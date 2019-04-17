/*
 * @Info:
 *      Answer of StackOverflow motion blur
 * 
 * @Author: Ricx8 
 * @Date: 04-17-2019 7:51:00 am 
 * @Last Modified by: Ricx8
 * @Last Modified time: 04-17-2019 7:51:00 am 
 * @Command: ./sfml-app [x(0-1)] [y(0-1)] [n samples] [blur radius]
 */
#include <SFML/Graphics.hpp>
#include <iostream>
#include <stdlib.h>
#include <math.h>

int main(int argc, char *argv[]){
    sf::RenderWindow window(sf::VideoMode(600, 600), "StackOverflow MotionBlur");

    // Make a texture
    sf::Texture texture;
    if (texture.loadFromFile("image.png")){
        texture.setSmooth(true);

        // Make a sprite and attach the texture to it.
        sf::Sprite sprite;
        sprite.setTexture(texture);
        sprite.setPosition(50, 50);

        // Load the shader
        sf::Shader shader;
        sf::Vector2f dir = { (float)std::atof(argv[1]) , (float)std::atof(argv[2]) };
        if (!shader.loadFromFile("dMotionBlur_v05.frag", sf::Shader::Fragment)){
            std::cerr<< "Error while shaders" << std::endl;
            return -1;
        }

        while (window.isOpen()){
            sf::Event event;
            while (window.pollEvent(event)){
                if (event.type == sf::Event::Closed)
                    window.close();
            }

            window.clear(sf::Color(120,120,120));

            // Passing parameters to shader.
            shader.setUniform("dir", dir); //direction of blur
            shader.setUniform("nSamplesF", (float)std::atoi(argv[3])); // number of samples
            shader.setUniform("radius", (float)std::atof(argv[4]) ); //radius of blur

            window.draw(sprite, &shader);
            window.display();
        }
    }

    return 0;
}