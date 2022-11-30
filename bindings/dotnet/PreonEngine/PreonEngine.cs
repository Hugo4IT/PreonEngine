using Preon.Events;

namespace Preon;

public class PreonEngine
{
    private unsafe void* _inner;

    public PreonEngine()
    {
        unsafe
        {
            _inner = NativeMethods.PreonEngine__new();
        }
    }

    public void SetTree(PreonComponent tree)
    {
        unsafe
        {
            NativeMethods.PreonEngine__set_tree(_inner, tree._inner);
        }
    }

    public void Run(Action<PreonComponent, PreonEvent, int> callback)
    {
        PreonComponent component = PreonComponent.StartBuilder().Build();

        unsafe
        {
            NativeMethods.preon__run(_inner, (tree, @event, userEvents) => {
                component._inner = tree;
                callback(component, NativeMethods.Unbind(@event), 2);
            });
        }
    }

    public static void Init()
    {
        NativeMethods.preon__init();
    }
}