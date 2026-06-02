#include "pixl.h"

void _pixl_glfw_error_callback(int error, const char* description) {
    fprintf(stderr, "Pixl: GLFW error %d: %s\n", error, description);
}

RESULT(PixlWindow, PixlCreateWindowError) pixl_create_window(int width, int height, char* title) {
    if (glfwInit() == 0) {
	return (RESULT(PixlWindow, PixlCreateWindowError))
	    RESULT_FAILURE(PIXL_CREATE_WINDOW_ERROR_GLFW_INIT);
    }

    glfwSetErrorCallback(_pixl_glfw_error_callback);

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow *glfwWindow = glfwCreateWindow(width, height, title, NULL, NULL);
    if (!glfwWindow) {
	return (RESULT(PixlWindow, PixlCreateWindowError))
	    RESULT_FAILURE(PIXL_CREATE_WINDOW_ERROR_GLFW_CREATE_WINDOW);
    }
    glfwMakeContextCurrent(glfwWindow);
    gladLoadGL(glfwGetProcAddress);

    PixlWindow window;
    window.glfwWindow = glfwWindow;
    return (RESULT(PixlWindow, PixlCreateWindowError))
	RESULT_SUCCESS(window);
}
