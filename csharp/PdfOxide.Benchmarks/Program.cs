using System;
using System.Collections.Generic;
using System.Linq;
using BenchmarkDotNet.Reports;
using BenchmarkDotNet.Running;

namespace PdfOxide.Benchmarks
{
    /// <summary>
    /// PDF Oxide C# Bindings Performance Benchmark Suite
    ///
    /// This application runs comprehensive performance benchmarks for the pdf_oxide C# bindings,
    /// measuring critical operations across:
    /// - Element access and enumeration
    /// - Annotation type detection and property access
    /// - Text search performance
    /// - Memory allocation patterns
    /// - Disposal and cleanup overhead
    ///
    /// Usage:
    ///     dotnet run -c Release
    ///
    /// Options:
    ///     dotnet run -c Release -- --filter ElementBenchmarks
    ///     dotnet run -c Release -- --help
    ///
    /// Results:
    ///     Benchmark results are saved to BenchmarkDotNet.Artifacts/results/
    /// </summary>
    internal class Program
    {
        static void Main(string[] args)
        {
            // Run all benchmarks defined in this assembly
            // For development/profiling, you can use:
            //   var summary = BenchmarkRunner.Run<ElementBenchmarks>();
            //   var summary = BenchmarkRunner.Run<AnnotationBenchmarks>();
            //   var summary = BenchmarkRunner.Run<SearchBenchmarks>();

            Console.WriteLine("PDF Oxide C# Bindings - Performance Benchmark Suite");
            Console.WriteLine("====================================================");
            Console.WriteLine();
            Console.WriteLine("Running benchmarks for:");
            Console.WriteLine("  - ElementBenchmarks (12 benchmarks)");
            Console.WriteLine("  - AnnotationBenchmarks (14 benchmarks)");
            Console.WriteLine("  - SearchBenchmarks (17 benchmarks)");
            Console.WriteLine();
            Console.WriteLine("Total: 43 performance benchmarks");
            Console.WriteLine();

            var switcher = BenchmarkSwitcher.FromAssembly(typeof(Program).Assembly);
            var summaries = switcher.Run(args);

            Console.WriteLine();
            Console.WriteLine("====================================================");
            Console.WriteLine("Benchmark Results Summary");
            Console.WriteLine("====================================================");
            Console.WriteLine();

            var summaryList = summaries?.ToList() ?? new List<Summary>();

            if (summaryList.Count == 0)
            {
                Console.WriteLine("⚠️  No benchmarks were executed");
            }
            else
            {
                Console.WriteLine($"✅ {summaryList.Count} benchmark suites completed");
                Console.WriteLine($"   Total benchmark methods: 43");
            }

            Console.WriteLine();
            Console.WriteLine("Results saved to: BenchmarkDotNet.Artifacts/results/");
            Console.WriteLine();
            Console.WriteLine("Recommendations:");
            Console.WriteLine("  - Run benchmarks in Release mode (-c Release) for accurate results");
            Console.WriteLine("  - Close other applications to minimize noise");
            Console.WriteLine("  - Run multiple times to identify consistent patterns");
            Console.WriteLine("  - Compare against baseline results to detect regressions");
            Console.WriteLine();
            Console.WriteLine("To view detailed results, check the generated reports in:");
            Console.WriteLine("  BenchmarkDotNet.Artifacts/results/");
        }
    }
}
