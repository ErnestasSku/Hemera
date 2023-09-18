# Hemera

This is the place for experimenting with rust + wGPU. This project is aiming to implement wallpaper engine features specifically for Linux. This repo contains only the engine part, and not the Daemon/Wayland layer shell/IPC part of the project. 


Some basics are already implemented. Such as basic scenes (still early and will require more work). And a transition mechanism between scenes. 



### Example of transition between 2 "gif scenes".
![Transition example](./files/hemera_mk6YUsg03O.gif)


### Future plans

There are many features I still want to implement, such as effects/shaders on images in the scene. Making the transition between use a custom user defined shader instead of one of the existing ones.