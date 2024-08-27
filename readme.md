# Ray Tracer

## Objectifs

Dans ce projet, l'objectif est de mettre en œuvre la méthode de **ray tracing** pour rendre une image générée par ordinateur contenant plusieurs objets. Ce projet doit prendre en compte les aspects suivants :

- Création d'au moins 4 objets simples : une sphère, un cube, un plan plat et un cylindre.
- Capacité à changer la position d'un objet avant la création de l'image (par exemple, rendre une sphère avec son centre au point (1,1,1)).
- Possibilité de visualiser la même scène sous différents angles en déplaçant la caméra/point de vue.
- Gestion simple de la lumière, incluant différentes intensités de luminosité et la gestion des ombres.


## Utilisation

Une fois le fichier de configuration préparé, exécutez le ray tracer en spécifiant le fichier de configuration et le nom du fichier de sortie :

```bash
cargo run configs/audit00.txt output.ppm
```

### Configuration

Le ray tracer utilise un fichier de configuration pour définir les paramètres de rendu de l'image. Voici un exemple de fichier de configuration :

```plaintext
#Config file for ray traicing image rendering :

$$$ image_size :
1280 720

$$$ background_color (black, grey, etc):
black

$$$ light_position (coordinates (x, y, z) or default) :
default

$$$ light_intensity (low, medium, high):
medium

$$$ light_color (white, yellow, etc) :
black

$$$ camera_position (coordinates (x, y, z), north, east, south, or west):
north

$$$ camera_look_at (coordinates (x, y, z)):
(0,0,0)

$$$ camera_orientation (roll axis in degrees):
0

$$$ camera_fov (field of view):
90

$$$ camera_aspect_ratio (image aspect ratio):
1.0

$$$ shapes (type, color, location) : 
flateplane/grey/(0,0,0)
sphere/grey/(-25,15,-25)
sphere/purple/(0,13,-40)
sphere/grey/(25,10,-25)
sphere/grey/(-33,0,7)
sphere/grey/(25,0,25)
$$$ end_shape
```

### Fonctionnalités
**Création d'objets**

Vous pouvez créer différents objets en utilisant la section $$$ shapes dans le fichier de configuration. Chaque objet est défini par son type, sa couleur et sa position. Voici quelques exemples :

```plaintext
    Sphère : sphere/purple/(0,13,-40)
    Plan Plat : flateplane/grey/(0,0,0)
    Cube : cube/red/(5,5,5)
    Cylindre : cylinder/blue/(10,0,0)
```

**Changement de la luminosité**

Vous pouvez ajuster l'intensité de la lumière dans la scène en utilisant la clé `$$$ light_intensity` avec les valeurs `low`, `medium` ou `high`. Exemple :

```plaintext
$$$ light_intensity (low, medium, high):
medium
```

**Modification de la position et de l'angle de la caméra**

Pour changer la position de la caméra, utilisez la clé `$$$ camera_position`. Vous pouvez spécifier des coordonnées ou des directions prédéfinies (north, east, south, west).

```plaintext
$$$ camera_position (coordinates (x, y, z), north, east, south, or west):
north
```

Vous pouvez aussi ajuster l'angle de la caméra avec `$$$ camera_orientation` en spécifiant l'angle en degrés autour de l'axe de roulis.
