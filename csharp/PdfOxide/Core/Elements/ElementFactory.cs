using System;
using PdfOxide.Internal;

namespace PdfOxide.Core.Elements
{
    /// <summary>
    /// Factory for creating appropriate PdfElement subclass instances.
    /// </summary>
    /// <remarks>
    /// This factory creates the correct element type based on the element
    /// type constant returned from the native FFI layer.
    /// </remarks>
    internal static class ElementFactory
    {
        /// <summary>
        /// Creates an element of the appropriate type from a native handle.
        /// </summary>
        /// <param name="handle">The native element handle.</param>
        /// <returns>A PdfElement instance of the appropriate type, or null if unable to create.</returns>
        public static PdfElement Create(NativeHandle handle)
        {
            if (handle == null)
                return null;

            var elementType = NativeMethods.PdfElementGetType(handle.DangerousGetHandle());

            return elementType switch
            {
                0 => new TextElement(handle),           // ELEMENT_TYPE_TEXT
                1 => new ImageElement(handle),          // ELEMENT_TYPE_IMAGE
                2 => new PathElement(handle),           // ELEMENT_TYPE_PATH
                3 => new TableElement(handle),          // ELEMENT_TYPE_TABLE
                4 => new StructureElement(handle),      // ELEMENT_TYPE_STRUCTURE
                _ => null,
            };
        }
    }
}
