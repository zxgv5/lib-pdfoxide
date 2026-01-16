using System;
using PdfOxide.Geometry;
using PdfOxide.Internal;

namespace PdfOxide.Core.Elements
{
    /// <summary>
    /// Element type enumeration.
    /// </summary>
    public enum ElementType
    {
        /// <summary>Text element (words, characters).</summary>
        Text = 0,

        /// <summary>Image element (JPEG, PNG, etc).</summary>
        Image = 1,

        /// <summary>Path element (lines, curves, shapes).</summary>
        Path = 2,

        /// <summary>Table element (structured data).</summary>
        Table = 3,

        /// <summary>Structure element (logical structure).</summary>
        Structure = 4,
    }

    /// <summary>
    /// Abstract base class for PDF page elements.
    /// </summary>
    /// <remarks>
    /// <para>
    /// PdfElement represents a content element on a PDF page.
    /// Elements can be text, images, paths, tables, or structural elements.
    /// </para>
    /// <para>
    /// Use the specific subclasses (TextElement, ImageElement, etc.) for
    /// type-specific operations.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var elements = page.FindElements(ElementType.Text);
    /// foreach (var element in elements)
    /// {
    ///     if (element is TextElement text)
    ///     {
    ///         Console.WriteLine($"Text: {text.Content}");
    ///         Console.WriteLine($"Size: {text.FontSize}pt");
    ///     }
    /// }
    /// </code>
    /// </example>
    public abstract class PdfElement : IDisposable
    {
        protected NativeHandle _handle;
        protected bool _disposed;

        /// <summary>
        /// Gets the element type.
        /// </summary>
        /// <value>The type of element.</value>
        public abstract ElementType Type { get; }

        /// <summary>
        /// Gets the bounding box of the element in points.
        /// </summary>
        /// <value>The bounding rectangle.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the element has been disposed.</exception>
        public Rect BoundingBox
        {
            get
            {
                ThrowIfDisposed();
                NativeMethods.PdfElementGetBbox(_handle.DangerousGetHandle(),
                    out var x, out var y, out var width, out var height);
                return new Rect(x, y, width, height);
            }
        }

        /// <summary>
        /// Gets the left coordinate of the element in points.
        /// </summary>
        /// <value>The x coordinate.</value>
        public float Left => BoundingBox.X;

        /// <summary>
        /// Gets the top coordinate of the element in points.
        /// </summary>
        /// <value>The y coordinate.</value>
        public float Top => BoundingBox.Y;

        /// <summary>
        /// Gets the width of the element in points.
        /// </summary>
        /// <value>The width.</value>
        public float Width => BoundingBox.Width;

        /// <summary>
        /// Gets the height of the element in points.
        /// </summary>
        /// <value>The height.</value>
        public float Height => BoundingBox.Height;

        /// <summary>
        /// Gets the center point of the element.
        /// </summary>
        /// <value>The center point.</value>
        public Point Center => new Point(
            BoundingBox.X + BoundingBox.Width / 2,
            BoundingBox.Y + BoundingBox.Height / 2);

        /// <summary>
        /// Gets the area of the element in square points.
        /// </summary>
        /// <value>The area.</value>
        public float Area => BoundingBox.Width * BoundingBox.Height;

        protected PdfElement(NativeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        /// <summary>
        /// Disposes the element and releases native resources.
        /// </summary>
        public virtual void Dispose()
        {
            if (!_disposed)
            {
                _handle?.Dispose();
                _disposed = true;
            }
        }

        protected void ThrowIfDisposed()
        {
            if (_disposed)
                throw new ObjectDisposedException(GetType().Name);
        }
    }
}
