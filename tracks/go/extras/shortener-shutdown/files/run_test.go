// Part of the URL shortener project (tracks/go/extras/shortener-shutdown).
// Don't edit this file: `learn` overwrites it. Your code goes in run.go.
//
// These tests listen on 127.0.0.1 at a port the system picks: nothing leaves
// your machine.

package shortener

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"testing"
	"time"
)

// start runs Run in a goroutine on a fresh local port. It returns the base
// URL, the cancel function that asks Run to stop, and where Run's result goes.
func start(t *testing.T, h http.Handler) (string, context.CancelFunc, <-chan error) {
	t.Helper()
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("can't listen: %v", err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan error, 1)
	go func() {
		defer func() {
			if r := recover(); r != nil {
				done <- fmt.Errorf("panicked: %v", r)
			}
		}()
		done <- Run(ctx, ln, h)
	}()
	t.Cleanup(func() { cancel(); ln.Close() })
	return "http://" + ln.Addr().String(), cancel, done
}

// wait gives Run a few seconds to return.
func wait(t *testing.T, done <-chan error) error {
	t.Helper()
	select {
	case err := <-done:
		return err
	case <-time.After(5 * time.Second):
		t.Fatal("Run didn't return within 5 seconds of being cancelled")
		return nil
	}
}

var hello = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { io.WriteString(w, "hello") })

func TestRun(t *testing.T) {
	t.Run("c40_run_serves_requests", func(t *testing.T) {
		defer failOnPanic(t)
		url, _, done := start(t, hello)
		time.Sleep(50 * time.Millisecond)
		select {
		case err := <-done:
			t.Fatalf("Run returned before it was cancelled: %v", err)
		default:
		}
		client := http.Client{Timeout: 2 * time.Second}
		resp, err := client.Get(url + "/")
		if err != nil {
			t.Fatalf("GET: %v", err)
		}
		defer resp.Body.Close()
		body, _ := io.ReadAll(resp.Body)
		if resp.StatusCode != http.StatusOK || string(body) != "hello" {
			t.Fatalf("got %d %q, want 200 \"hello\"", resp.StatusCode, body)
		}
	})
	t.Run("c41_run_returns_nil_once_cancelled", func(t *testing.T) {
		defer failOnPanic(t)
		_, cancel, done := start(t, hello)
		time.Sleep(50 * time.Millisecond)
		cancel()
		if err := wait(t, done); err != nil {
			t.Fatalf("Run returned %v, want nil after a clean shutdown", err)
		}
	})
	t.Run("c42_a_request_in_flight_still_gets_its_answer", func(t *testing.T) {
		defer failOnPanic(t)
		started := make(chan struct{})
		h := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			close(started)
			time.Sleep(300 * time.Millisecond)
			io.WriteString(w, "done")
		})
		url, cancel, done := start(t, h)
		answer := make(chan string, 1)
		go func() {
			client := http.Client{Timeout: 5 * time.Second}
			resp, err := client.Get(url + "/slow")
			if err != nil {
				answer <- "error: " + err.Error()
				return
			}
			defer resp.Body.Close()
			body, _ := io.ReadAll(resp.Body)
			answer <- string(body)
		}()
		select {
		case <-started:
		case err := <-done:
			t.Fatalf("Run returned before serving: %v", err)
		case <-time.After(5 * time.Second):
			t.Fatal("the request never reached the handler")
		}
		cancel()
		if got := <-answer; got != "done" {
			t.Fatalf("the request in flight got %q, want \"done\"", got)
		}
		if err := wait(t, done); err != nil {
			t.Fatalf("Run returned %v, want nil", err)
		}
	})
	t.Run("c43_new_connections_are_refused_after_shutdown", func(t *testing.T) {
		defer failOnPanic(t)
		url, cancel, done := start(t, hello)
		time.Sleep(50 * time.Millisecond)
		cancel()
		if err := wait(t, done); err != nil {
			t.Fatalf("Run returned %v", err)
		}
		client := http.Client{Timeout: 2 * time.Second}
		if resp, err := client.Get(url + "/"); err == nil {
			resp.Body.Close()
			t.Fatalf("a request after shutdown got %d, want the connection refused", resp.StatusCode)
		}
	})
}
