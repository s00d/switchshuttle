#!/usr/bin/env swift
import AppKit
import Foundation

/// Regenerates macOS tray-menu template PNGs from SF Symbols.
/// Usage: swift scripts/render-menu-icons.swift [outDir]

let outDir = CommandLine.arguments.count > 1
  ? CommandLine.arguments[1]
  : "src-tauri/icons"

let pointSize: CGFloat = 18
let pixelSize = 72

struct Spec {
  let file: String
  let symbol: String
}

let icons: [Spec] = [
  .init(file: "document", symbol: "doc.text"),
  .init(file: "edit", symbol: "square.and.pencil"),
  .init(file: "folder", symbol: "folder"),
  .init(file: "visual", symbol: "macwindow"),
  .init(file: "refresh_settings", symbol: "arrow.clockwise"),
  .init(file: "config", symbol: "gearshape"),
  .init(file: "info", symbol: "info.circle"),
  .init(file: "help", symbol: "questionmark.circle"),
  .init(file: "site", symbol: "globe"),
  .init(file: "exit", symbol: "power"),
  .init(file: "stop", symbol: "stop.circle"),
  .init(file: "terminal", symbol: "terminal"),
  .init(file: "create", symbol: "plus.circle"),
  .init(file: "yes", symbol: "checkmark"),
  .init(file: "no", symbol: "xmark"),
  .init(file: "update", symbol: "arrow.triangle.2.circlepath"),
  .init(file: "devtools", symbol: "wrench.and.screwdriver"),
]

func render(symbol: String) -> NSBitmapImageRep? {
  let config = NSImage.SymbolConfiguration(pointSize: pointSize, weight: .medium)
  guard let base = NSImage(systemSymbolName: symbol, accessibilityDescription: nil),
        let symbolImage = base.withSymbolConfiguration(config) else { return nil }

  guard let rep = NSBitmapImageRep(
    bitmapDataPlanes: nil,
    pixelsWide: pixelSize,
    pixelsHigh: pixelSize,
    bitsPerSample: 8,
    samplesPerPixel: 4,
    hasAlpha: true,
    isPlanar: false,
    colorSpaceName: .deviceRGB,
    bytesPerRow: 0,
    bitsPerPixel: 0
  ) else { return nil }

  rep.size = NSSize(width: pixelSize, height: pixelSize)
  NSGraphicsContext.saveGraphicsState()
  NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
  defer { NSGraphicsContext.restoreGraphicsState() }

  NSColor.clear.setFill()
  NSRect(x: 0, y: 0, width: pixelSize, height: pixelSize).fill()

  let pad: CGFloat = 8
  let box = NSRect(
    x: pad,
    y: pad,
    width: CGFloat(pixelSize) - pad * 2,
    height: CGFloat(pixelSize) - pad * 2
  )
  let src = symbolImage.size
  let scale = min(box.width / max(src.width, 1), box.height / max(src.height, 1))
  let draw = NSSize(width: src.width * scale, height: src.height * scale)
  let origin = NSPoint(x: box.midX - draw.width / 2, y: box.midY - draw.height / 2)
  symbolImage.draw(
    in: NSRect(origin: origin, size: draw),
    from: .zero,
    operation: .sourceOver,
    fraction: 1.0
  )
  return rep
}

var failed = 0
for icon in icons {
  guard let rep = render(symbol: icon.symbol),
        let data = rep.representation(using: .png, properties: [:]) else {
    fputs("FAIL \(icon.symbol)\n", stderr)
    failed += 1
    continue
  }
  let path = (outDir as NSString).appendingPathComponent("\(icon.file).png")
  try data.write(to: URL(fileURLWithPath: path))
  print("OK \(icon.file) <= \(icon.symbol)")
}

exit(failed == 0 ? 0 : 1)
