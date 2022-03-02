#include <GL/glew.h>
#include <GL/glut.h>
#include <GL/gl.h>
#include <GL/glu.h>
#include <GL/glext.h>

GLenum err = glewInit();
if (GLEW_OK != err)
{
    fprintf(stderr, "Error %s\n", glewGetErrorString(err));
    exit(1);
}
fprintf(stdout, "Status: Using GLEW %s\n", glewGetString(GLEW_VERSION));

if (GLEW_ARB_vertex_program)
    fprintf(stdout, "Status: ARB vertex programs available.\n");

if (glewGetExtension("GL_ARB_fragment_program"))
    fprintf(stdout, "Status: ARB fragment programs available.\n");

if (glewIsSupported("GL_VERSION_1_4  GL_ARB_point_sprite"))
    fprintf(stdout, "Status: ARB point sprites available.\n");
