using System;
using PdfOxide.Exceptions;
using PdfOxide.Internal;

namespace PdfOxide.Core.Annotations
{
    /// <summary>
    /// Represents a link annotation on a PDF page.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Link annotations create clickable regions that navigate to URLs or
    /// other pages in the PDF. They support both external URLs and internal
    /// page destinations.
    /// </para>
    /// </remarks>
    /// <example>
    /// <code>
    /// var link = annotation as LinkAnnotation;
    /// if (link != null)
    /// {
    ///     string uri = link.Uri;
    ///     if (!string.IsNullOrEmpty(uri))
    ///     {
    ///         Console.WriteLine($"Link to: {uri}");
    ///     }
    ///     else
    ///     {
    ///         int page = link.DestinationPage;
    ///         if (page >= 0)
    ///             Console.WriteLine($"Link to page: {page}");
    ///     }
    /// }
    /// </code>
    /// </example>
    public sealed class LinkAnnotation : Annotation
    {
        /// <summary>
        /// Gets the annotation type.
        /// </summary>
        public override AnnotationType Type => AnnotationType.Link;

        /// <summary>
        /// Gets the URI if this is a URI link.
        /// </summary>
        /// <value>The URI, or empty string if this is not a URI link.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public string Uri
        {
            get
            {
                ThrowIfDisposed();
                var ptr = NativeMethods.PdfLinkAnnotationGetUri(_handle.DangerousGetHandle(),
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
        /// Gets the destination page index if this is an internal page link.
        /// </summary>
        /// <value>The page index, or -1 if not a page link or error.</value>
        /// <exception cref="ObjectDisposedException">Thrown if the annotation has been disposed.</exception>
        public int DestinationPage
        {
            get
            {
                ThrowIfDisposed();
                var page = NativeMethods.PdfLinkAnnotationGetPage(_handle.DangerousGetHandle(),
                    out var errorCode);
                ExceptionMapper.ThrowIfError(errorCode);
                return page;
            }
        }

        /// <summary>
        /// Gets whether this link points to a URI.
        /// </summary>
        /// <value>True if this is a URI link, false if it's an internal link.</value>
        public bool IsUriLink => !string.IsNullOrEmpty(Uri);

        /// <summary>
        /// Gets whether this link points to an internal page.
        /// </summary>
        /// <value>True if this is an internal page link, false otherwise.</value>
        public bool IsPageLink => DestinationPage >= 0;

        internal LinkAnnotation(NativeHandle handle) : base(handle)
        {
        }
    }
}
