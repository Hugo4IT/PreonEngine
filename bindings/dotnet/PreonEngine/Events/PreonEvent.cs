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
    public class Button : PreonEvent
    {
        public uint Id { get; set; }
        public PreonButtonState State { get; set; }
    }
}