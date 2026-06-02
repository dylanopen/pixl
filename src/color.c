#include "pixl.h"

PixlColor pixl_color_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    return (PixlColor) {
	.r = r,
	.g = g,
	.b = b,
	.a = a,
    };
}
