initSidebarItems({"enum":[["Action","Input actions."],["ClientApiHint","Client API tokens."],["ContextCreationApi","Specifies the API to use to create the context"],["ContextReleaseBehavior","`ContextReleaseBehavior` specifies the release behavior to be used by the context."],["ContextRobustnessHint","Context robustness tokens."],["CursorMode","Cursor modes."],["Error","Tokens corresponding to various error types."],["GamepadAxis","Axis identifier tokens."],["GamepadButton","Button identifier tokens."],["InitError","An error that might be returned when `glfw::init` is called."],["InitHint","Initialization hints that can be set using the `init_hint` function."],["JoystickEvent","Joystick events."],["JoystickId","Joystick identifier tokens."],["Key","Input keys."],["MonitorEvent","Monitor events."],["MouseButton","Mouse buttons. The `MouseButtonLeft`, `MouseButtonRight`, and `MouseButtonMiddle` aliases are supplied for convenience."],["OpenGlProfileHint","OpenGL profile tokens."],["StandardCursor","Standard cursors provided by GLFW"],["SwapInterval","Specifies how the context should handle swapping the buffers."],["WindowEvent","Window event messages."],["WindowHint","Window hints that can be set using the `window_hint` function."],["WindowMode","Describes the mode of a window"]],"fn":[["fail_on_errors","The function to be used with the `FAIL_ON_ERRORS` callback."],["flush_messages","Returns an iterator that yields until no more messages are contained in the `Receiver`'s queue. This is useful for event handling where the blocking behaviour of `Receiver::iter` is undesirable."],["get_key_name","Wrapper around 'glfwGetKeyName`"],["get_key_scancode","Wrapper around `glfwGetKeyScancode`."],["get_version","Wrapper for `glfwGetVersion`."],["get_version_string","Wrapper for `glfwGetVersionString`."],["init","Initializes the GLFW library. This must be called on the main platform thread."],["init_hint","Sets hints for the next initialization of GLFW."],["key_name","Wrapper around 'glfwGetKeyName`"],["log_errors","The function to be used with the `LOG_ERRORS` callback."],["make_context_current","Wrapper for `glfwMakeContextCurrent`."],["string_from_c_str","Replacement for `String::from_raw_buf`"],["string_from_nullable_c_str","Like `string_from_c_str`, but handles null pointers correctly"],["with_c_str","Replacement for `ToCStr::with_c_str`"]],"mod":[["ffi","Low-level function bindings and constants pertaining to the underlying GLFW library."]],"static":[["FAIL_ON_ERRORS","A callback that triggers a task failure when an error is encountered."],["LOG_ERRORS","A callback that logs each error as it is encountered without triggering a task failure."]],"struct":[["Callback",""],["Cursor","Represents a window cursor that can be used to display any of the standard cursors or load a custom cursor from an image."],["DebugAliases","Formats the type using aliases rather than the default variant names."],["FlushedMessages","An iterator that yields until no more messages are contained in the `Receiver`'s queue."],["GamepadState","State of a gamepad."],["GammaRamp","Describes the gamma ramp of a monitor."],["Glfw","A token from which to call various GLFW functions. It can be obtained by calling the `init` function. This cannot be sent to other tasks, and should only be initialized on the main platform thread. Whilst this might make performing some operations harder, this is to ensure thread safety is enforced statically."],["Joystick","A joystick handle."],["JoystickHats","Joystick hats."],["Modifiers","Key modifiers (e.g., Shift, Control, Alt, Super)"],["Monitor","A struct that wraps a `*GLFWmonitor` handle."],["PixelImage","When not using the `image` library, or if you just want to, you can specify an image from its raw pixel data using this structure."],["RenderContext","A rendering context that can be shared between tasks."],["VidMode","Describes a single video mode."],["Window","A struct that wraps a `*GLFWwindow` handle."]],"trait":[["Context","Methods common to renderable contexts"]],"type":[["ErrorCallback","An error callback. This can be supplied with some user data to be passed to the callback function when it is triggered."],["GLProc","An OpenGL process address."],["JoystickCallback","An joystick callback. This can be supplied with some user data to be passed to the callback function when it is triggered."],["MonitorCallback","An monitor callback. This can be supplied with some user data to be passed to the callback function when it is triggered."],["Scancode","Keyboard code returned by the OS"],["VkProc","A Vulkan process address"],["WindowId","Unique identifier for a `Window`."]]});