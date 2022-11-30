using System.Runtime.InteropServices;

namespace Preon.Types;

public class PreonCorners
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct Inner
    {
        int _topLeft, _topRight, _bottomRight, _bottomLeft;
    }

    internal Inner _inner;
}