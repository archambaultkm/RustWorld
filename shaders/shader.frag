#version 330 core

out vec4 fragColour;

in vec2 texCoord; // Texture coordinates for the cube
uniform float atlasX;
uniform float atlasY;
uniform float atlasW;
uniform float atlasH;
uniform sampler2D blockAtlas; // Texture atlas containing multiple cube textures
uniform int blockType; // Type of block (dirt, stone, etc.) // TODO handle control flow on block types in here probably

const vec2 atlasSize = vec2(322.0 , 486.0);

void main() {
    // Determine the offset and size of the block's texture in the atlas based on blockType.
    // This could be done with conditional statements or a lookup table.
    // TODO handle control flow on block types in here probably

    vec4 blockCoords = vec4(atlasX/atlasSize.y, atlasY/atlasSize.x, atlasW/atlasSize.y, atlasH/atlasSize.x);

    // Sample the appropriate region of the atlas using adjusted texture coordinates.
    vec4 blockColour = texture(blockAtlas, texCoord.xy * blockCoords.zw + blockCoords.xy);

    fragColour = blockColour;
}