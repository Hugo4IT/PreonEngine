using System.Runtime.InteropServices;

namespace Preon.Types;

public class PreonVector<T>
{
    public T X { get { return _inner._x; } set { _inner._x = value; } }
    public T Y { get { return _inner._y; } set { _inner._y = value; } }
        
    [StructLayout(LayoutKind.Sequential)]
    internal struct Inner
    {
        internal T _x, _y;
    }

    internal Inner _inner;

    public PreonVector(T x, T y)
    {
        _inner._x = x;
        _inner._y = y;
    }
}