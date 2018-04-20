# Maiko:

## Introduction:
I started out by doing the tutorials from [this e-book](https://doc.rust-lang.org/stable/book/second-edition/ch01-00-introduction.html). 


## Findings:
Rust is a pretty hard language to write, because of all the things that the compiler wants you to guarantee.

## My setup:
I started out with Sublime Text 3 and the powershell. It worked well for following along with the book because I didn't have to look a lot additional documention up. 
But when I was starting to write my own OpenGL program it started to get more tough.
Because when all the compile errors were displaying in the terminal it would take a relatively long time to check where that error was. 
And when I was writing my abstractions for the window struct, I also needed to know the types of the variables, because previously they were deducted by the compiler on the return type:
```
{
	// Without abstraction:
	let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
	    .expect("Failed to create GLFW window.");
}

{
	// Abstraction:
	struct Window{
		handle: glfw::Window,
		events: ??? Some event handler???,
	}

	impl Window {
		fn new(title: &String) -> Window{
			let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
				.expect("Failed to create GLFW window.");

			Window{
				window: window,
				events: events,
			}
		}
	}
}
```

## Rust build script
For building rust we only had to use `cargo build` and `cargo run`. However this built the exe in target/%configuration%/%proj_name%.exe and would execute that in the working directory the executable itself.
After searching around I found two solutions:
1. Add the feature to the build.rs file, which would copy the resource directory to the build directory.
2. Use a custom build task in VS Code.
