using Preon;
using Preon.Types;
using Preon.Events;

namespace Demo;

public static class PreonComponentBuilderInputFieldExtension
{
    private static int InputFieldID = 0;

    public delegate void InputFieldChangedCallback(PreonComponent component, string content);

    public static PreonComponentBuilder StartInputField(
            this PreonComponentBuilder builder,
            string? id = null,
            string placeholder = "",
            PreonColor? placeholderColor = null,
            InputFieldChangedCallback? onChanged = null)
    {
        // Apply defaults
        string componentId = id ?? $"__inputField{InputFieldID++}";
        PreonColor componentPlaceholderColor = placeholderColor ?? new PreonColor(0.0f, 0.0f, 0.0f, 0.5f);

        // State
        bool isFocused = false;
        string inputBuffer = "";

        // Callbacks

        builder.EngineRef.OnWindowOpened += () =>
        {
            PreonComponent component = builder.EngineRef.Tree.GetChildById(componentId);
            component.Text = placeholder;
        };

        builder.EngineRef.OnMouseInput += (index, state) =>
        {
            if (isFocused && state == PreonButtonState.Pressed)
                isFocused = false;
        };

        builder.EngineRef.OnPressed(componentId, (component, state) =>
        {
            if (state == PreonButtonState.Pressed)
                isFocused = true;
        });

        // Using PreonKeyCode
        builder.EngineRef.OnKeyboardInput += (key, state) =>
        {
            if (!isFocused) return;
            if (state != PreonButtonState.Pressed) return;

            PreonComponent component = builder.EngineRef.Tree.GetChildById(componentId);

            if (key == PreonKeyCode.Backspace && inputBuffer.Length >= 1)
            {
                inputBuffer = inputBuffer.Substring(0, inputBuffer.Length - 1);

                component.Text = (inputBuffer.Length == 0) ? placeholder : inputBuffer;
                onChanged?.Invoke(component, inputBuffer);
                builder.EngineRef.ForceUpdate();
            }
        };

        // Using IMEs
        builder.EngineRef.OnReceivedCharacter += ch =>
        {
            if (!isFocused) return;
            if (Char.IsControl(ch)) return;

            PreonComponent component = builder.EngineRef.Tree.GetChildById(componentId);

            inputBuffer += ch;

            component.Text = (inputBuffer.Length == 0) ? placeholder : inputBuffer;
            onChanged?.Invoke(component, inputBuffer);
            builder.EngineRef.ForceUpdate();
        };

        // View
        return builder
            .StartPanel(PreonColor.White)
                .Id(componentId)

                .MinSize(new PreonVector<int>(0, 48))
                .Padding(new PreonBorder(8, 16, 8, 16))
                .ExpandHorizontally()

                .BackgroundColor(PreonColor.White)
                .ForegroundColor(componentPlaceholderColor)
                
                .ReceiveEvents(true);
    }
}