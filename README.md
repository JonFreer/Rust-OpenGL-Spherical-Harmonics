# Shperical Harmonic Lighting Dataset Generation

Loads in directory of .ply models and renders out 2^15 images per model altering the spherical harmonic lighting coefficients.

## Use 

- -i, --input: input model .ply
- -o, --ouput: output directory for images
- -n, --name: name of ouput

```cargo run -- -i=E:\Jon\rust_opengl\game\assets\models\res2.ply -o=E:\Jon\rust_opengl\out -n=Jon```