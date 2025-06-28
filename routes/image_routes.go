package routes

import (
	"neko-love/handlers"

	"github.com/gofiber/fiber/v2"
)

// SetupRoutes registers image-related routes to the provided Fiber router group.
// It sets up endpoints for serving random images from the "neko" and "hug" asset directories.
//
// Routes:
//   GET /neko - Returns a random image from the "assets/neko" directory.
//   GET /hug  - Returns a random image from the "assets/hug" directory.
//
// Parameters:
//   v4 fiber.Router - The Fiber router group to which the routes will be attached.
func SetupRoutes(v4 fiber.Router) {
	v4.Get("/neko", handlers.GetRandomImage("assets/neko"))
	v4.Get("/hug", handlers.GetRandomImage("assets/hug"))
}