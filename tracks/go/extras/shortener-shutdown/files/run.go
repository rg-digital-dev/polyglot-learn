package shortener

import (
	"context"
	"net"
	"net/http"
)

// Run serves h on ln (use an http.Server and its Serve method, in a goroutine)
// until ctx is done. Then it shuts down gracefully with Shutdown: no new
// connections, and requests in flight get up to 5 seconds to finish. Run
// returns nil after a clean shutdown, and Serve's error if serving fails
// before that.
//
// To try the whole service, a main package could do:
//
//	ln, _ := net.Listen("tcp", "localhost:8080")
//	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
//	defer stop()
//	store := shortener.NewStore(time.Now)
//	defer store.StartJanitor(time.Minute)()
//	h := shortener.Chain(shortener.NewServer(store), shortener.Recover, shortener.RequestID,
//		shortener.Logger(os.Stderr), shortener.Timeout(5*time.Second))
//	log.Fatal(shortener.Run(ctx, ln, h))
func Run(ctx context.Context, ln net.Listener, h http.Handler) error {
	panic("TODO: implement Run")
}
