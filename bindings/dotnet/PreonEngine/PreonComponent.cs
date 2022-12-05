namespace Preon;

public class PreonComponent
{
    public string Text
    {
        get { unsafe { return NativeMethods.PreonComponent__get_text(_inner); } }
        set { unsafe { NativeMethods.PreonComponent__set_text(_inner, value); } }
    }

    internal unsafe void* _inner;

    internal unsafe PreonComponent(void* ptr)
    {
        _inner = ptr;
    }

    public void AddChild(PreonComponent child)
    {
        unsafe
        {
            NativeMethods.PreonComponent__add_child(_inner, child._inner);
        }
    }


    public void InsertChild(ushort index, PreonComponent child)
    {
        unsafe
        {
            NativeMethods.PreonComponent__insert_child(_inner, index, child._inner);
        }
    }

    public void RemoveChild(ushort index)
    {
        unsafe
        {
            NativeMethods.PreonComponent__remove_child(_inner, index);
        }
    }

    public void ClearChildren()
    {
        unsafe
        {
            NativeMethods.PreonComponent__clear_children(_inner);
        }
    }

    public PreonComponent GetChildById(string id)
    {
        unsafe
        {
            return new PreonComponent(NativeMethods.PreonComponent__get_child_ref_mut_by_id(_inner, id));
        }
    }

    public static PreonComponentBuilder StartBuilder(PreonEngine engine)
    {
        return new PreonComponentBuilder(engine);
    }

    ~PreonComponent()
    {
        Console.WriteLine("Destructor");
    }
}