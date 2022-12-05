using System.Runtime.InteropServices;

namespace Preon.Types;

[StructLayout(LayoutKind.Sequential)]
public struct PreonCorners
{
    public float topLeft, topRight, bottomRight, bottomLeft;
}