use std::ffi::CStr;

const VERTEX_SHADER: &CStr = cr#"
#version 330 core

uniform mat4 projection;

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aSize;
layout(location = 2) in vec2 aPosition;
layout(location = 3) in vec4 aColor;
layout(location = 4) in vec4 aBorderColor;
layout(location = 5) in float aBorderSize;

flat out vec4 vertexColor;
flat out vec4 borderColor;
flat out float borderSize;
out vec2 localPos;
flat out vec2 elementSize;

void main(){
    
  vec2 aFinal = aPos * aSize + aPosition;
  
  gl_Position = projection * vec4(aFinal,0.0,1.0); 
  vertexColor = aColor;

  borderColor = aBorderColor;
  borderSize = aBorderSize;
  localPos = aPos;
  elementSize = aSize;
}"#;
const VERTEX_SHADER_PTR: *const i8 = VERTEX_SHADER.as_ptr();
pub const VERTEX_SHADER_PTR_PTR: *const *const i8 = &VERTEX_SHADER_PTR;

const FRAGMENT_SHADER: &CStr = cr#"
#version 330 core

flat in vec4 vertexColor;
flat in vec4 borderColor;
flat in float borderSize;
in vec2 localPos;
flat in vec2 elementSize;


out vec4 fragColor;

void main() {
    vec2 edgeDistance =
        min(localPos, vec2(1.0) - localPos) * elementSize;

    if (edgeDistance.x < borderSize ||
        edgeDistance.y < borderSize) {
        fragColor = vec4(borderColor.x/255.0,borderColor.y/255.0,borderColor.z/255.0,borderColor.w);
    } else {
        fragColor = vec4(vertexColor.x/255.0,vertexColor.y/255.0,vertexColor.z/255.0,vertexColor.w);
    }   
}"#;
const FRAGMENT_SHADER_PTR: *const i8 = FRAGMENT_SHADER.as_ptr();
pub const FRAGMENT_SHADER_PTR_PTR: *const *const i8 = &FRAGMENT_SHADER_PTR;