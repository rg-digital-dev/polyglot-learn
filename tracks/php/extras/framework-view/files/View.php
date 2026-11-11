<?php
// Plain PHP templates, rendered into a string, with escaping by default.

declare(strict_types=1);

/**
 * Text made safe to put in HTML, attributes included: <, >, &, " and ' all
 * escaped, and broken UTF-8 replaced with U+FFFD rather than dropped.
 */
function e(mixed $value): string
{
    throw new RuntimeException('TODO: implement e');
}

final class View
{
    public function __construct(private readonly string $dir)
    {
    }

    /**
     * Render $template (a file in the template folder) with $vars as local
     * variables, plus $view (this View) for layouts, and return the output
     * instead of printing it. If the template called $view->layout(), render
     * that layout next with the same variables, its own, and $content.
     * A missing file throws RuntimeException("no template <name>"). An
     * exception from a template comes out of render with no output buffer
     * left open.
     */
    public function render(string $template, array $vars = []): string
    {
        throw new RuntimeException('TODO: implement View::render');
    }

    /** Called from a template: wrap this template's output in $file. */
    public function layout(string $file, array $vars = []): void
    {
        throw new RuntimeException('TODO: implement View::layout');
    }
}
