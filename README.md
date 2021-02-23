# Shperical Harmonic Lighting Dataset Generation

Loads in .ply model and renders out 2^15 images per model altering the spherical harmonic lighting coefficients.

Dataset generation for: https://vision.uvic.ca/pubs/2017/mandl2017learning/paper.pdf

## Use 

- -i, --input: input model .ply
- -o, --ouput: output directory for images
- -n, --name: name of ouput

```cargo run -- -i=E:\Jon\rust_opengl\game\assets\models\res2.ply -o=E:\Jon\rust_opengl\out -n=Jon```