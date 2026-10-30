package embeddedlogger

// Logger is satisfied by any type that has these two methods. Nothing has to
// declare that it implements it.
type Logger interface {
	Log(msg string)
	Lines() []string
}

// Base holds the behaviour every logger shares. Log stores Prefix + msg, and
// Lines returns the stored lines in order, as a copy the caller may change.
type Base struct {
	Prefix string
	lines  []string
}

func (b *Base) Log(msg string) {
	panic("TODO: implement Base.Log")
}

func (b *Base) Lines() []string {
	panic("TODO: implement Base.Lines")
}

// Plain logs every message unchanged. Embed Base so Plain gets Log and Lines
// without writing them again.
type Plain struct {
	// TODO: embed Base
}

// Shouty logs every message in upper case with "!" at the end; the prefix is
// kept as it is. Embed Base and give Shouty its own Log, which hides the
// promoted one.
type Shouty struct {
	// TODO: embed Base
}

// NewPlain returns a *Plain with the given prefix.
func NewPlain(prefix string) Logger {
	panic("TODO: implement NewPlain")
}

// NewShouty returns a *Shouty with the given prefix.
func NewShouty(prefix string) Logger {
	panic("TODO: implement NewShouty")
}

// Broadcast logs msg to every logger.
func Broadcast(msg string, loggers ...Logger) {
	panic("TODO: implement Broadcast")
}
