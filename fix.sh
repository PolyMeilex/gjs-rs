#!/bin/bash
set -x -e

xmlstarlet ed -L \
    -d '//_:function[@c:identifier="gjs_profiler_get_type"]' \
    -u '//_:type[@name="gboolean" and @c:type="_Bool"]/@c:type' -v "gboolean" \
    -a '//_:record[@name="Profiler" and not(@glib:get-type)]' -t attr -n "glib:get-type" -v "gjs_profiler_get_type" \
    -a '//_:record/_:function/_:parameters/_:parameter/_:type[@c:type="siginfo_t*"]' -t attr -n name -v "gpointer" \
    -u '//_:record/_:function/_:parameters/_:parameter/_:type[@c:type="siginfo_t*"]/@c:type' -v "void*" \
    Gjs-1.0.gir

