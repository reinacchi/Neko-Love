package handlers

import (
	"math/rand"
	"os"
	"path/filepath"
	"time"

	"github.com/gofiber/fiber/v2"
)

// GetRandomImage returns a Fiber handler that serves a random image file from the specified folder.
// The handler reads all files in the given folder, selects one at random, and responds with a JSON
// object containing the URL to the selected image. If no images are found or an error occurs while
// reading the directory, it responds with a 500 status and an error message.
//
// Parameters:
//   - folder: The path to the directory containing image files.
//
// Returns:
//   - fiber.Handler: An HTTP handler function for serving a random image URL.
func GetRandomImage(folder string) fiber.Handler {
	return func(c *fiber.Ctx) error {
		files, err := os.ReadDir(folder)
		if err != nil || len(files) == 0 {
			return c.Status(500).JSON(fiber.Map{"error": "no images found"})
		}

		rand.Seed(time.Now().UnixNano())
		randomFile := files[rand.Intn(len(files))].Name()
		category := filepath.Base(folder)

		return c.JSON(fiber.Map{
			"url": "/static/" + category + "/" + randomFile,
		})
	}
}
