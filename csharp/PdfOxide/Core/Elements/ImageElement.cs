using System;
using PdfOxide.Exceptions;
using PdfOxide.Geometry;
using PdfOxide.Internal;

namespace PdfOxide.Core.Elements
{
    /// <summary>
    /// Image format enumeration.
    /// </summary>
    public enum ImageFormat
    {
        /// <summary>JPEG format.</summary>
        Jpeg = 0,

        /// <summary>PNG format.</summary>
        Png = 1,

        /// <summary>JPEG 2000 format (JPX).</summary>
        Jpeg2000 = 2,

        /// <summary>JBIG2 format (typically for scanned documents).</summary>
        Jbig2 = 3,

        /// <summary>Raw uncompressed image data.</summary>
        Raw = 4,

        /// <summary>Unknown or unsupported format.</summary>
        Unknown = 5,
    }

    /// <summary>
    /// Represents an image element on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// ImageElement provides access to image data, format, dimensions,
    /// and metadata such as DPI and alt text.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var image = element as ImageElement;
    /// if (image != null)
    /// {
    ///     Console.WriteLine($"Format: {image.Format}");
    ///     Console.WriteLine($"Size: {image.Width}x{image.Height}");
    ///     
    ///     byte[] data = image.ImageData;
    ///     File.WriteAllBytes("extracted.jpg", data);
    /// }
    /// </code>
    /// </example>
    public sealed class ImageElement : PdfElement
    {
        /// <summary>
        /// Gets the element type.
        /// </summary>
        public override ElementType Type => ElementType.Image;

        /// <summary>
        /// Gets the image format.
        /// </summary>
        /// <value>The image format.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the element has been disposed.</exception>
        public ImageFormat Format
        {
            get
            {
                ThrowIfDisposed();
                var format = NativeMethods.PdfImageElementGetFormat(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return (ImageFormat)format;
            }
        }

        /// <summary>
        /// Gets the image dimensions in pixels (width, height).
        /// </summary>
        /// <value>A tuple of width and height.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the element has been disposed.</exception>
        public (int Width, int Height) Dimensions
        {
            get
            {
                ThrowIfDisposed();
                NativeMethods.PdfImageElementGetDimensions(_handle.DangerousGetHandle(),
                    out var width, out var height, out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return ((int)width, (int)height);
            }
        }

        /// <summary>
        /// Gets the image aspect ratio (width / height).
        /// </summary>
        /// <value>The aspect ratio.</value>
        public float AspectRatio
        {
            get
            {
                var (width, height) = Dimensions;
                if (height == 0)
                    return 0;
                return width / (float)height;
            }
        }

        /// <summary>
        /// Gets the image data as a byte array.
        /// </summary>
        /// <value>The raw image bytes.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the element has been disposed.</exception>
        public byte[] ImageData
        {
            get
            {
                ThrowIfDisposed();

                // Get the size of the image data
                var size = NativeMethods.PdfImageElementGetDataSize(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);

                if (size <= 0)
                    return Array.Empty<byte>();

                // Extract the image data
                var data = new byte[size];
                var bytesRead = NativeMethods.PdfImageElementGetData(_handle.DangerousGetHandle(),
                    data, size, out errorCode);
                ExceptionMapper.ThrowIfError(errorCode);

                if (bytesRead < 0 || bytesRead > size)
                    return Array.Empty<byte>();

                // If fewer bytes were read than expected, resize the array
                if (bytesRead < size)
                {
                    System.Array.Resize(ref data, bytesRead);
                }

                return data;
            }
        }

        /// <summary>
        /// Gets the alternative text for the image.
        /// </summary>
        /// <value>The alt text, or null if not set.</value>
        public string AltText => null; // Placeholder

        /// <summary>
        /// Gets the horizontal DPI (dots per inch) of the image.
        /// </summary>
        /// <value>The horizontal DPI, or null if not available.</value>
        public float? HorizontalDpi => null; // Placeholder

        /// <summary>
        /// Gets the vertical DPI (dots per inch) of the image.
        /// </summary>
        /// <value>The vertical DPI, or null if not available.</value>
        public float? VerticalDpi => null; // Placeholder

        /// <summary>
        /// Gets whether the image is grayscale.
        /// </summary>
        public bool IsGrayscale => false; // Placeholder

        internal ImageElement(NativeHandle handle) : base(handle)
        {
        }
    }
}
