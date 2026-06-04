#include "lib/gl.c"
#include "pixl.h"

GLuint _pixl_shader_type_to_gl(PixlShaderType type) {
    switch (type) {
	case PixlVertexShaderType:
	    return GL_VERTEX_SHADER;
	case PixlFragmentShaderType:
	    return GL_FRAGMENT_SHADER;
    }
    return NULL; // unreachable
}

PixlShader pixl_load_shader(const char* source, PixlShaderType type) {
    // TODO: error handling
    GLuint gl = glCreateShader(_pixl_shader_type_to_gl(type));
    glShaderSource( gl, 1, &source, NULL );
    glCompileShader( gl );
    return (PixlShader) {
	.source = source,
	.gl = gl,
    };
}

PixlShaderProgram pixl_create_shader_program(int number_of_shaders, PixlShader shaders[]) {
    // TODO: error handling
    GLuint gl = glad_glCreateProgram();
    for (int i = 0; i < number_of_shaders; i++) {
	glad_glAttachShader(gl, shaders[i].gl);
    }
    glad_glLinkProgram(gl);
}

