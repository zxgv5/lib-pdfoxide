using System;
using PdfOxide.Exceptions;
using PdfOxide.Geometry;
using PdfOxide.Internal;

namespace PdfOxide.Core.Elements
{
    /// <summary>
    /// Represents a text element on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// TextElement provides access to text content and formatting properties
    /// such as font name, size, and color.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var text = element as TextElement;
    /// if (text != null)
    /// {
    ///     Console.WriteLine($"Content: {text.Content}");
    ///     Console.WriteLine($"Font: {text.FontName} {text.FontSize}pt");
    ///     Console.WriteLine($"Color: {text.Color}");
    /// }
    /// </code>
    /// </example>
    public sealed class TextElement : PdfElement
    {
        /// <summary>
        /// Gets the element type.
        /// </summary>
        public override ElementType Type => ElementType.Text;

        /// <summary>
        /// Gets the text content.
        /// </summary>
        /// <value>The text string.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the element has been disposed.</exception>
        /// <exception cref="PdfException">Thrown if content cannot be retrieved.</exception>
        public string Content
        {
            get
            {
                ThrowIfDisposed();
                var ptr = NativeMethods.PdfTextElementGetContent(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);

                if (ptr == IntPtr.Zero)
                    return string.Empty;

                try
                {
                    return StringMarshaler.PtrToString(ptr);
                }
                finally
                {
                    NativeMethods.FreeString(ptr);
                }
            }
        }

        /// <summary>
        /// Gets the font size in points.
        /// </summary>
        /// <value>The font size.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the element has been disposed.</exception>
        public float FontSize
        {
            get
            {
                ThrowIfDisposed();
                var size = NativeMethods.PdfTextElementGetFontSize(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return size;
            }
        }

        /// <summary>
        /// Gets the font name.
        /// </summary>
        /// <value>The font name (e.g., "Helvetica", "Times-Roman").</value>
        public string FontName => "Arial"; // Placeholder - would get from FFI

        /// <summary>
        /// Gets the text color.
        /// </summary>
        /// <value>The color in RGB.</value>
        public Color Color => Color.Black; // Placeholder - would get from FFI

        /// <summary>
        /// Gets whether the text is bold.
        /// </summary>
        public bool IsBold => false; // Placeholder

        /// <summary>
        /// Gets whether the text is italic.
        /// </summary>
        public bool IsItalic => false; // Placeholder

        internal TextElement(NativeHandle handle) : base(handle)
        {
        }
    }
}
