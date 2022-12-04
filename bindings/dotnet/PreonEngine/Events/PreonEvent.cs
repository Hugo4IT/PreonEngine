using Preon.Types;

namespace Preon.Events;

public class PreonEvent
{
    public class WindowResized : PreonEvent
    {
        public PreonVector<uint> NewSize { get; internal set; } = new PreonVector<uint>(0, 0);
    }

    public class WindowOpened : PreonEvent {}
    public class WindowClosed : PreonEvent {}
    public class Update : PreonEvent {}
    public class LayoutUpdate : PreonEvent {}
    public class ComponentPressed : PreonEvent
    {
        public string Id { get; set; }
        public PreonButtonState State { get; set; }
    }
    public class MouseInput : PreonEvent
    {
        public ushort Index { get; set; }
        public PreonButtonState State { get; set; }
    }
    public class KeyboardInput : PreonEvent
    {
        public PreonKeyCode Key { get; set; }
        public PreonButtonState State { get; set; }
    }
    public class ReceivedCharacter : PreonEvent
    {
        public char Char;
    }
}