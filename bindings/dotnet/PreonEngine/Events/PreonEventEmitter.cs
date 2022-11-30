namespace Preon.Events;

public class PreonEventEmitter
{
    private unsafe void* _inner;

    internal unsafe PreonEventEmitter(void* inner)
    {
        _inner = inner;
    }
}