using System;
using PdfOxide.Exceptions;
using PdfOxide.Geometry;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Annotation type enumeration.
    /// </summary>
    public enum AnnotationType
    {
        /// <summary>Text annotation (sticky note).</summary>
        Text = 0,

        /// <summary>Link annotation.</summary>
        Link = 1,

        /// <summary>FreeText annotation (text box).</summary>
        FreeText = 2,

        /// <summary>Line annotation.</summary>
        Line = 3,

        /// <summary>Square annotation.</summary>
        Square = 4,

        /// <summary>Circle annotation.</summary>
        Circle = 5,

        /// <summary>Polygon annotation.</summary>
        Polygon = 6,

        /// <summary>PolyLine annotation.</summary>
        PolyLine = 7,

        /// <summary>Highlight annotation.</summary>
        Highlight = 8,

        /// <summary>Underline annotation.</summary>
        Underline = 9,

        /// <summary>Squiggly annotation.</summary>
        Squiggly = 10,

        /// <summary>StrikeOut annotation.</summary>
        StrikeOut = 11,

        /// <summary>Stamp annotation.</summary>
        Stamp = 12,

        /// <summary>Caret annotation.</summary>
        Caret = 13,

        /// <summary>Ink annotation (freehand drawing).</summary>
        Ink = 14,

        /// <summary>Popup annotation.</summary>
        Popup = 15,

        /// <summary>FileAttachment annotation.</summary>
        FileAttachment = 16,

        /// <summary>Sound annotation.</summary>
        Sound = 17,

        /// <summary>Movie annotation (legacy).</summary>
        Movie = 18,

        /// <summary>Widget annotation (form field).</summary>
        Widget = 19,

        /// <summary>Screen annotation.</summary>
        Screen = 20,

        /// <summary>PrinterMark annotation.</summary>
        PrinterMark = 21,

        /// <summary>TrapNet annotation.</summary>
        TrapNet = 22,

        /// <summary>Watermark annotation.</summary>
        Watermark = 23,

        /// <summary>3D annotation.</summary>
        ThreeD = 24,

        /// <summary>Redact annotation.</summary>
        Redact = 25,

        /// <summary>RichMedia annotation.</summary>
        RichMedia = 26,

        /// <summary>Unknown annotation type.</summary>
        Unknown = 27,
    }

    /// <summary>
    /// Annotation flag bits for controlling annotation behavior.
    /// </summary>
    [Flags]
    public enum AnnotationFlags
    {
        /// <summary>No flags set.</summary>
        None = 0,

        /// <summary>If set, do not display if no appearance stream.</summary>
        Invisible = 1 << 0,

        /// <summary>If set, do not display and do not print.</summary>
        Hidden = 1 << 1,

        /// <summary>If set, print the annotation.</summary>
        Print = 1 << 2,

        /// <summary>If set, do not scale annotation's appearance with zoom.</summary>
        NoZoom = 1 << 3,

        /// <summary>If set, do not rotate annotation's appearance with page.</summary>
        NoRotate = 1 << 4,

        /// <summary>If set, do not display annotation on screen.</summary>
        NoView = 1 << 5,

        /// <summary>If set, do not allow annotation to be deleted or modified.</summary>
        ReadOnly = 1 << 6,

        /// <summary>If set, do not allow annotation to be deleted or modified.</summary>
        Locked = 1 << 7,

        /// <summary>If set, toggle NoView when mouse enters/exits.</summary>
        ToggleNoView = 1 << 8,

        /// <summary>If set, do not allow editing of annotation contents.</summary>
        LockedContents = 1 << 9,
    }

    /// <summary>
    /// Represents an annotation on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Annotation is the abstract base class for all PDF annotations.
    /// Use the specific subclasses (TextAnnotation, LinkAnnotation, etc.)
    /// for type-specific operations.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var annotations = page.FindAnnotations();
    /// foreach (var annotation in annotations)
    /// {
    ///     Console.WriteLine($"Type: {annotation.Type}");
    ///     Console.WriteLine($"Contents: {annotation.Contents}");
    ///     Console.WriteLine($"Author: {annotation.Author}");
    /// }
    /// </code>
    /// </example>
    public abstract class Annotation : IDisposable
    {
        protected NativeHandle _handle;
        protected bool _disposed;

        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        /// <value>The type of annotation.</value>
        public abstract AnnotationType Type { get; }

        /// <summary>
        /// Gets the contents (text) of the annotation.
        /// </summary>
        /// <value>The annotation contents, or empty string if not set.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public string Contents
        {
            get
            {
                ThrowIfDisposed();
                var ptr = NativeMethods.PdfAnnotationGetContents(_handle.DangerousGetHandle(),
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
        /// Gets the subject of the annotation.
        /// </summary>
        /// <value>The subject line, or empty string if not set.</value>
        public string Subject
        {
            get
            {
                ThrowIfDisposed();
                var ptr = NativeMethods.PdfAnnotationGetSubject(_handle.DangerousGetHandle(),
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
        /// Gets the author of the annotation.
        /// </summary>
        /// <value>The author name, or empty string if not set.</value>
        public string Author
        {
            get
            {
                ThrowIfDisposed();
                var ptr = NativeMethods.PdfAnnotationGetAuthor(_handle.DangerousGetHandle(),
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
        /// Gets the bounding box of the annotation in points.
        /// </summary>
        /// <value>The bounding rectangle.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public Rect BoundingBox
        {
            get
            {
                ThrowIfDisposed();
                NativeMethods.PdfAnnotationGetBbox(_handle.DangerousGetHandle(),
                    out var x, out var y, out var width, out var height);
                return new Rect(x, y, width, height);
            }
        }

        /// <summary>
        /// Gets the left coordinate of the annotation in points.
        /// </summary>
        public float Left => BoundingBox.X;

        /// <summary>
        /// Gets the top coordinate of the annotation in points.
        /// </summary>
        public float Top => BoundingBox.Y;

        /// <summary>
        /// Gets the width of the annotation in points.
        /// </summary>
        public float Width => BoundingBox.Width;

        /// <summary>
        /// Gets the height of the annotation in points.
        /// </summary>
        public float Height => BoundingBox.Height;

        /// <summary>
        /// Gets the center point of the annotation.
        /// </summary>
        public Point Center => new Point(
            BoundingBox.X + BoundingBox.Width / 2,
            BoundingBox.Y + BoundingBox.Height / 2);

        /// <summary>
        /// Gets the opacity of the annotation (0.0-1.0).
        /// </summary>
        /// <value>The opacity value (1.0 = fully opaque).</value>
        public float Opacity
        {
            get
            {
                ThrowIfDisposed();
                var opacity = NativeMethods.PdfAnnotationGetOpacity(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return opacity;
            }
        }

        /// <summary>
        /// Gets the color of the annotation in RGB format.
        /// </summary>
        /// <value>The color, or null if not set.</value>
        public Color? Color
        {
            get
            {
                ThrowIfDisposed();
                NativeMethods.PdfAnnotationGetColor(_handle.DangerousGetHandle(),
                    out var r, out var g, out var b, out var hasColor);
                
                if (hasColor == 0)
                    return null;

                return new Color((byte)(r * 255), (byte)(g * 255), (byte)(b * 255));
            }
        }

        /// <summary>
        /// Gets the annotation flags.
        /// </summary>
        public AnnotationFlags Flags
        {
            get
            {
                ThrowIfDisposed();
                var flags = NativeMethods.PdfAnnotationGetFlags(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return (AnnotationFlags)flags;
            }
        }

        protected Annotation(NativeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        /// <summary>
        /// Disposes the annotation and releases native resources.
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
