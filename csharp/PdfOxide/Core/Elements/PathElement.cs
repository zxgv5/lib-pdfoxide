using System;
using PdfOxide.Exceptions;
using PdfOxide.Geometry;
using PdfOxide.Internal;

namespace PdfOxide.Core.Elements
{
    /// <summary>
    /// Path fill mode enumeration.
    /// </summary>
    public enum PathFillMode
    {
        /// <summary>No fill (stroke only).</summary>
        None = 0,

        /// <summary>Non-zero winding rule fill.</summary>
        NonZeroWinding = 1,

        /// <summary>Even-odd fill rule.</summary>
        EvenOdd = 2,
    }

    /// <summary>
    /// Path stroke style enumeration.
    /// </summary>
    public enum PathStrokeStyle
    {
        /// <summary>No stroke.</summary>
        None = 0,

        /// <summary>Solid stroke.</summary>
        Solid = 1,

        /// <summary>Dashed stroke.</summary>
        Dashed = 2,

        /// <summary>Dotted stroke.</summary>
        Dotted = 3,

        /// <summary>Dash-dot stroke.</summary>
        DashDot = 4,
    }

    /// <summary>
    /// Represents a path element (vector graphic) on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// PathElement represents vector graphics including lines, curves, shapes,
    /// and complex paths. Paths can have strokes, fills, or both, and support
    /// various line styles.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var path = element as PathElement;
    /// if (path != null)
    /// {
    ///     Console.WriteLine($"BBox: {path.BoundingBox}");
    ///     Console.WriteLine($"Stroke Color: {path.StrokeColor}");
    ///     Console.WriteLine($"Fill Color: {path.FillColor}");
    ///     Console.WriteLine($"Line Width: {path.LineWidth}pt");
    /// }
    /// </code>
    /// </example>
    public sealed class PathElement : PdfElement
    {
        /// <summary>
        /// Gets the element type.
        /// </summary>
        public override ElementType Type => ElementType.Path;

        /// <summary>
        /// Gets the stroke color of the path.
        /// </summary>
        /// <value>The stroke color in RGB, or null if not stroked.</value>
        public Color? StrokeColor => Color.Black; // Placeholder

        /// <summary>
        /// Gets the fill color of the path.
        /// </summary>
        /// <value>The fill color in RGB, or null if not filled.</value>
        public Color? FillColor => null; // Placeholder

        /// <summary>
        /// Gets the line width in points.
        /// </summary>
        /// <value>The line width, or 0 if not stroked.</value>
        public float LineWidth => 1.0f; // Placeholder

        /// <summary>
        /// Gets the fill mode for the path.
        /// </summary>
        public PathFillMode FillMode => PathFillMode.NonZeroWinding; // Placeholder

        /// <summary>
        /// Gets the stroke style for the path.
        /// </summary>
        public PathStrokeStyle StrokeStyle => PathStrokeStyle.Solid; // Placeholder

        /// <summary>
        /// Gets whether the path is stroked.
        /// </summary>
        public bool IsStroked => LineWidth > 0;

        /// <summary>
        /// Gets whether the path is filled.
        /// </summary>
        public bool IsFilled => FillColor.HasValue;

        internal PathElement(NativeHandle handle) : base(handle)
        {
        }
    }
}
