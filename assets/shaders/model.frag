#version 330 core

uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 sharm[16];
const float M_PI = 3.14159265358979323846;
const float contrast = 0.2f;
const float brightnes = -0.5f;
in VS_OUTPUT {
    vec3 Color;
    vec3 Normal;
    vec3 Position;
} IN;

out vec4 Color;


//Taken from https://github.com/jingyangcarl/SphericalHarmonicsLighting/blob/master/SphericalHarmonicsLighting/Object.fsh
vec3 sph_harm(vec3 normal){
        
		// spherical harmonics lighting
		float x = normal.x;
		float y = normal.y;
		float z = normal.z;
		float x2 = x * x;
		float y2 = y * y;
		float z2 = z * z;

		// basis 
		float basis[16];
		// level 0
		basis[0] = 1.0f / 2.0f * sqrt(1.0f / M_PI); // check
		// level 1
		basis[1] = sqrt(3.0f / (4.0f * M_PI)) * y; // check
		basis[2] = sqrt(3.0f / (4.0f * M_PI)) * z; // check
		basis[3] = sqrt(3.0f / (4.0f * M_PI)) * x; // check
		// level 2
		basis[4] = 1.0f / 2.0f * sqrt(15.0f / M_PI) * x * y; // check
		basis[5] = 1.0f / 2.0f * sqrt(15.0f / M_PI) * z * y; // check
		basis[6] = 1.0f / 4.0f * sqrt(5.0f / M_PI) * (-x2 - y2 + 2.0f*z2); // check
		basis[7] = 1.0f / 2.0f * sqrt(15.0f / M_PI) * x * z; // check
		basis[8] = 1.0f / 4.0f * sqrt(15.0f / M_PI) * (x2 - y2); // check
		// level 3
		basis[9] = 1.0f / 4.0f * sqrt(35.0f / (2.0f * M_PI)) * (3.0f*x2 - y2) * y; // check
		basis[10] = 1.0f / 2.0f * sqrt(105.0f / M_PI) * x * z * y; // check
		basis[11] = 1.0f / 4.0f * sqrt(21.0f / (2.0f * M_PI)) * y * (5.0f*z2 - x2 - y2); // check
		basis[12] = 1.0f / 4.0f * sqrt(7.0f / M_PI) * z * (1.5f*z2 - 3.0f*x2 - 3.0f*y2); // check
		basis[13] = 1.0f / 4.0f * sqrt(21.0f / (2.0f * M_PI)) * x * (5.0f*z2 - x2 - y2); // check
		basis[14] = 1.0f / 4.0f * sqrt(105.0f / M_PI) * (x2 - y2) * z; // check
		basis[15] = 1.0f / 4.0f * sqrt(35.0f / (2.0f * M_PI)) * (x2 - 3.0f*y2) * x; // check
        
        // spherical harmonic lighting
		vec3 shColor = vec3(0.0, 0.0, 0.0);
		for (int i = 0; i < 16; i++) shColor += sharm[i].x * basis[i];

		// shColor.x = max(0,shColor.x);
		// shColor.y = max(0,shColor.y);
		// shColor.z = max(0,shColor.z);
        // contrast
		shColor = (shColor - 0.5f) * contrast + 0.5f;
		// brightnes
		shColor += brightnes;

		return shColor;


}

void main()
{

    Color = vec4(sph_harm(normalize(IN.Normal))+IN.Color,1.0f); //Add here but can also mul
}