using Preon.Events;
using Preon.Types;

namespace Preon;

public class PreonEngine
{
    public delegate void ResizedCallback(PreonVector<uint> newSize);
    public delegate void ButtonCallback(PreonComponent pressed, PreonButtonState state);

    private unsafe void* _inner;
    private PreonComponent? _tree;

    public event ResizedCallback? OnResized;
    public Dictionary<string, ButtonCallback> _buttonCallbacks;
    public PreonComponent Tree
    {
        get { return _tree; }
        set { unsafe { NativeMethods.PreonEngine__set_tree(_inner, value._inner); } }
    }


    public PreonEngine()
    {
        _buttonCallbacks = new();

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

    public void OnPressed(string id, ButtonCallback callback)
    {
        _buttonCallbacks.Add(id, callback);
    }

    public void Run()
    {
        _tree = PreonComponent.StartBuilder().Build();

        unsafe
        {
            NativeMethods.preon__run(_inner, (tree, @event, userEvents) => {
                _tree._inner = tree;

                switch (NativeMethods.Unbind(@event))
                {
                    case PreonEvent.ComponentPressed pressedEvent:
                        if (_buttonCallbacks.TryGetValue(pressedEvent.Id, out ButtonCallback? buttonCallback))
                            buttonCallback(_tree.GetChildById(pressedEvent.Id), pressedEvent.State);
                        break;
                    case PreonEvent.WindowResized resizedEvent:
                        OnResized?.Invoke(resizedEvent.NewSize);
                        break;
                }
            });
        }
    }

    public static void Init()
    {
        NativeMethods.preon__init();
    }
}