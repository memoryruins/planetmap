#version 450

#include "terrain.h"

layout(location = 0) in vec2 texcoords;
layout(location = 1) in vec3 base_normal;
layout(location = 2) in vec3 tangent;
layout(location = 3) in flat uint slot;

layout(location = 0) out vec4 color;

void main() {
    vec3 base_normal_ = normalize(base_normal);
    vec3 tangent_ = normalize(tangent);
    vec3 bitangent = cross(base_normal_, tangent_);
    mat3 tangent_basis = mat3(tangent_, bitangent, base_normal_);

    uint array = slot / CACHE_ARRAY_SIZE;
    uint layer = slot % CACHE_ARRAY_SIZE;
    vec2 encoded = texture(normals[array], vec3(texcoords, layer)).xy;
    vec3 decoded = vec3(encoded, sqrt(1-dot(encoded.xy, encoded.xy)));
    vec3 normal = tangent_basis * decoded;

    vec4 base_color = texture(colors[array], vec3(texcoords, layer));

    vec3 sun = mat3(view) * vec3(0, 1, 0);
    color = base_color * dot(normal, sun);
}
