using System.Runtime.InteropServices;

namespace Preon.Types;

public class PreonColor
{
    public static readonly PreonColor White = new PreonColor(1.0f, 1.0f, 1.0f);

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

    public PreonColor(byte r, byte g, byte b)
    {
        _inner._r = MathF.Pow(((float)r) / 255.0f, 2.2f);
        _inner._g = MathF.Pow(((float)g) / 255.0f, 2.2f);
        _inner._b = MathF.Pow(((float)b) / 255.0f, 2.2f);
        _inner._a = 1.0f;
    }

    public PreonColor(byte r, byte g, byte b, byte a)
    {
        _inner._r = MathF.Pow(((float)r) / 255.0f, 2.2f);
        _inner._g = MathF.Pow(((float)g) / 255.0f, 2.2f);
        _inner._b = MathF.Pow(((float)b) / 255.0f, 2.2f);
        _inner._a = ((float)a) / 255.0f;
    }
}