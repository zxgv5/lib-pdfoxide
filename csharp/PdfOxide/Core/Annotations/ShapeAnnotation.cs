using System;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Represents a shape annotation (square, circle, line, polygon) on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Shape annotations draw geometric shapes like rectangles, circles, lines,
    /// and polygons on the page. They support stroke and fill colors with
    /// customizable styling.
    /// </para>
    /// </remarks>
    public sealed class ShapeAnnotation : Annotation
    {
        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        public override AnnotationType Type
        {
            get
            {
                var rawType = NativeMethods.PdfAnnotationGetType(_handle.DangerousGetHandle());
                return rawType switch
                {
                    4 => AnnotationType.Square,
                    5 => AnnotationType.Circle,
                    3 => AnnotationType.Line,
                    6 => AnnotationType.Polygon,
                    7 => AnnotationType.PolyLine,
                    _ => AnnotationType.Unknown,
                };
            }
        }

        /// <summary>
        /// Gets whether this is a square annotation.
        /// </summary>
        public bool IsSquare => Type == AnnotationType.Square;

        /// <summary>
        /// Gets whether this is a circle annotation.
        /// </summary>
        public bool IsCircle => Type == AnnotationType.Circle;

        /// <summary>
        /// Gets whether this is a line annotation.
        /// </summary>
        public bool IsLine => Type == AnnotationType.Line;

        /// <summary>
        /// Gets whether this is a polygon annotation.
        /// </summary>
        public bool IsPolygon => Type == AnnotationType.Polygon;

        /// <summary>
        /// Gets whether this is a polyline annotation.
        /// </summary>
        public bool IsPolyLine => Type == AnnotationType.PolyLine;

        internal ShapeAnnotation(NativeHandle handle) : base(handle)
        {
        }
    }
}
