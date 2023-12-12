#version 330 core

out vec4 fragColour;

in vec2 texCoord; // Texture coordinates for the cube
uniform int blockType; // Type of block
uniform sampler2D blockAtlas; // Texture atlas containing multiple cube textures

const vec2 atlasSize = vec2(322.0 , 486.0);

float atlasX;
float atlasY;
float atlasH;
float atlasW;

void main() {
    // Determine the offset and size of the block's texture in the atlas based on blockType.
    switch (blockType) {
        case 0: // AIR
        break;

        case 1: // GRASS
        atlasX = 163;
        atlasY = 163;
        atlasW = 160;
        atlasH = 158;
        break;

        case 2: // STONE
        atlasX = 325;
        atlasY = 1;
        atlasW = 160;
        atlasH = 160;
        break;

        case 3: // DIRT
        atlasX = 1;
        atlasY = 1;
        atlasW = 160;
        atlasH = 161;
        break;
    }

    // calculate the coordinates of the texture in the atlas
    vec4 blockCoords = vec4(atlasX/atlasSize.y, atlasY/atlasSize.x, atlasW/atlasSize.y, atlasH/atlasSize.x);

    // Sample the appropriate region of the atlas using adjusted texture coordinates.
    vec4 blockColour = texture(blockAtlas, texCoord.xy * blockCoords.zw + blockCoords.xy);

    fragColour = blockColour;
}