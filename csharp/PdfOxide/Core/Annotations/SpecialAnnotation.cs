using System;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Represents special annotation types (stamps, popups, ink, file attachments, etc.).
    /// </summary>
    /// <remarks>
    /// <para>
    /// This class represents less common annotation types including:
    /// - Stamp annotations (Approved, Draft, etc.)
    /// - Popup annotations (associated with other annotations)
    /// - Ink annotations (freehand drawings)
    /// - FileAttachment annotations (embedded files)
    /// - Redact annotations (content redaction markers)
    /// - Watermark annotations (background text)
    /// - Sound and Movie annotations (multimedia)
    /// - 3D and RichMedia annotations (advanced content)
    /// </para>
    /// </remarks>
    public sealed class SpecialAnnotation : Annotation
    {
        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        public override AnnotationType Type
        {
            get
            {
                var rawType = NativeMethods.PdfAnnotationGetType(_handle.DangerousGetHandle());
                return (AnnotationType)rawType;
            }
        }

        /// <summary>
        /// Gets whether this is a stamp annotation.
        /// </summary>
        public bool IsStamp => Type == AnnotationType.Stamp;

        /// <summary>
        /// Gets whether this is a popup annotation.
        /// </summary>
        public bool IsPopup => Type == AnnotationType.Popup;

        /// <summary>
        /// Gets whether this is an ink annotation (freehand drawing).
        /// </summary>
        public bool IsInk => Type == AnnotationType.Ink;

        /// <summary>
        /// Gets whether this is a file attachment annotation.
        /// </summary>
        public bool IsFileAttachment => Type == AnnotationType.FileAttachment;

        /// <summary>
        /// Gets whether this is a redact annotation.
        /// </summary>
        public bool IsRedact => Type == AnnotationType.Redact;

        /// <summary>
        /// Gets whether this is a watermark annotation.
        /// </summary>
        public bool IsWatermark => Type == AnnotationType.Watermark;

        /// <summary>
        /// Gets whether this is a caret annotation.
        /// </summary>
        public bool IsCaret => Type == AnnotationType.Caret;

        /// <summary>
        /// Gets whether this is a sound annotation.
        /// </summary>
        public bool IsSound => Type == AnnotationType.Sound;

        /// <summary>
        /// Gets whether this is a movie annotation.
        /// </summary>
        public bool IsMovie => Type == AnnotationType.Movie;

        /// <summary>
        /// Gets whether this is a widget (form field) annotation.
        /// </summary>
        public bool IsWidget => Type == AnnotationType.Widget;

        /// <summary>
        /// Gets whether this is a screen annotation.
        /// </summary>
        public bool IsScreen => Type == AnnotationType.Screen;

        /// <summary>
        /// Gets whether this is a 3D annotation.
        /// </summary>
        public bool IsThreeD => Type == AnnotationType.ThreeD;

        /// <summary>
        /// Gets whether this is a RichMedia annotation.
        /// </summary>
        public bool IsRichMedia => Type == AnnotationType.RichMedia;

        internal SpecialAnnotation(NativeHandle handle) : base(handle)
        {
        }
    }
}
