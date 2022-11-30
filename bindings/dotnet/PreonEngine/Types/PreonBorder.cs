using System.Runtime.InteropServices;

namespace Preon.Types;

public class PreonBorder
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct Inner
    {
        internal int _top, _right, _bottom, _left;
    }

    internal Inner _inner;

    public PreonBorder(int top, int right, int bottom, int left)
    {
        _inner._top = top;
        _inner._right = right;
        _inner._bottom = bottom;
        _inner._left = left;
    }
}