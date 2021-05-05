# Shperical Harmonic Lighting Dataset Generation

Loads in .ply model and renders out 2^15 images per model altering the spherical harmonic lighting coefficients.

Dataset generation for: https://vision.uvic.ca/pubs/2017/mandl2017learning/paper.pdf

## Use 

- -i, --input: input model .ply
- -o, --ouput: output directory for images
- -n, --name: name of ouput
- -f, --freq: frequency of harmonics

```cargo run -- -i=E:\Jon\PIFu\res\result_res90.ply -o=E:\Jon\rust_opengl\val -n=Jon2 -f=1```

```cargo run -- -i=E:\\Jon\\PIFu\\results\\google2\\result_res1.ply -o=E:\Jon\rust_opengl\tmp -n=Jon2 -f=1 -s=0 -h="0.994_0.999_-0.122_0.939_0.732_0.4_0.177_0.326_-0.105_0.044_0.3_-0.166_-0.033_0.065_0.058"```