#include "pixl.h"

GLuint _pixl_shader_type_to_gl(PixlShaderType type) {
    switch (type) {
	case PixlVertexShaderType:
	    return GL_VERTEX_SHADER;
	case PixlFragmentShaderType:
	    return GL_FRAGMENT_SHADER;
    }
}

PixlShader pixl_load_shader(const char* source, PixlShaderType type) {
    GLuint gl = glCreateShader(_pixl_shader_type_to_gl(type));
    glShaderSource( gl, 1, &source, NULL );
    glCompileShader( gl );
    return (PixlShader) {
	.source = source,
	.gl = gl,
    };
}

