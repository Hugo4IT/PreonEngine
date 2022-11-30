namespace Preon.Events;

public class PreonUserEvent
{
    public enum Kind : byte
    {
        WindowResized = 0, //(PreonVector<u32>),
        WindowOpened = 1,
        WindowClosed = 2,
        MouseMove = 3, //(PreonVector<i32>),
        ForceUpdate = 4,
    }

    public Kind EventKind { get; private set; }
    public object? Data { get; private set; }
}