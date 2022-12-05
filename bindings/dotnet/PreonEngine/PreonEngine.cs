using Preon.Events;
using Preon.Types;

namespace Preon;

public class PreonEngine
{
    public delegate void GenericCallback();
    public delegate void ReceivedCharacterCallback(char ch);
    public delegate void KeyboardInputCallback(PreonKeyCode key, PreonButtonState state);
    public delegate void MouseInputCallback(ushort index, PreonButtonState state);
    public delegate void ResizedCallback(PreonVector<uint> newSize);
    public delegate void ButtonCallback(PreonComponent pressed, PreonButtonState state);

    private unsafe void* _inner;
    private PreonComponent? _tree;
    private bool _forceUpdate;

    public event GenericCallback? OnWindowOpened;
    public event GenericCallback? OnWindowClosed;
    public event GenericCallback? OnUpdate;
    public event GenericCallback? OnLayoutUpdate;
    public event ResizedCallback? OnResized;
    public event MouseInputCallback? OnMouseInput;
    public event KeyboardInputCallback? OnKeyboardInput;
    public event ReceivedCharacterCallback? OnReceivedCharacter;
    public Dictionary<string, ButtonCallback> _buttonCallbacks;

    public PreonComponent Tree
    {
        get { return _tree!; }
        set { _tree = value; unsafe { NativeMethods.PreonEngine__set_tree(_inner, value._inner); } }
    }


    public PreonEngine()
    {
        _buttonCallbacks = new();

        unsafe
        {
            _inner = NativeMethods.PreonEngine__new();
        }

        OnWindowOpened += () => ForceUpdate();
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
        if (!_buttonCallbacks.TryAdd(id, callback))
            _buttonCallbacks[id] = callback;
    }

    public void OnPressedRemove(string id)
    {
        _buttonCallbacks.Remove(id);
    }

    public void ForceUpdate()
    {
        _forceUpdate = true;
    }

    public void Run()
    {
        unsafe
        {
            _tree = new PreonComponent(null);
        
            NativeMethods.preon__run(_inner, (tree, @event, userEvents) => {
                _tree._inner = tree;
                _forceUpdate = false;

                switch (NativeMethods.Unbind(@event))
                {
                    case PreonEvent.ComponentPressed pressedEvent:
                        if (_buttonCallbacks.TryGetValue(pressedEvent.Id, out ButtonCallback? buttonCallback))
                            buttonCallback.Invoke(_tree.GetChildById(pressedEvent.Id), pressedEvent.State);
                        break;
                    
                    case PreonEvent.WindowResized resizedEvent: OnResized?.Invoke(resizedEvent.NewSize); break;
                    case PreonEvent.MouseInput mouseInputEvent: OnMouseInput?.Invoke(mouseInputEvent.Index, mouseInputEvent.State); break;
                    case PreonEvent.KeyboardInput keyboardInputEvent: OnKeyboardInput?.Invoke(keyboardInputEvent.Key, keyboardInputEvent.State); break;
                    case PreonEvent.ReceivedCharacter receivedCharacterEvent: OnReceivedCharacter?.Invoke(receivedCharacterEvent.Char); break;
                    
                    case PreonEvent.WindowOpened _: OnWindowOpened?.Invoke(); break;
                    case PreonEvent.WindowClosed _: OnWindowClosed?.Invoke(); break;
                    case PreonEvent.Update _:       OnUpdate?.Invoke();       break;
                    case PreonEvent.LayoutUpdate _: OnLayoutUpdate?.Invoke(); break;
                }

                return _forceUpdate;
            });
        }
    }

    public static void Init()
    {
        NativeMethods.preon__init();
    }
}