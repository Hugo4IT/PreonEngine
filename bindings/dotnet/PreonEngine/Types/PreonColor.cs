using System.Runtime.InteropServices;

namespace Preon.Types;

public class PreonColor
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct Inner
    {
        internal float _r, _g, _b, _a;
    }

    internal Inner _inner;

    public PreonColor(float r, float g, float b, float a)
    {
        _inner._r = r;
        _inner._g = g;
        _inner._b = b;
        _inner._a = a;
    }

    public PreonColor(float r, float g, float b)
    {
        _inner._r = r;
        _inner._g = g;
        _inner._b = b;
        _inner._a = 1.0f;
    }
}