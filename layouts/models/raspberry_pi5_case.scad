// Raspberry Pi 5 rectangular case
// Generated as a parametric OpenSCAD starter model.
// Units: millimeters
//
// Outer size requested by user:
//   8" W x 5" H x 5" D
// Converted to:
//   203.2 x 127 x 127 mm
//
// Notes:
// - Left side: fan opening + 4 mounting holes
// - Right side: airflow vent holes
// - Top: sliding lid riding in side channels
// - Back: HDMI + power pass-through slots
// - Front: opening for a 3.5" Pi display
// - Optional Pi standoffs included; verify spacing/position before printing
// - Fan and screen mounting dimensions vary by part/vendor, so key values are parameters below.

inch = 25.4;
$fn = 48;

// =========================
// Main dimensions
// =========================
outer_w = 8 * inch;      // left-right
outer_h = 5 * inch;      // bottom-top
outer_d = 5 * inch;      // front-back

wall = 3;
bottom_t = 3;
lid_t = 3;
lid_clearance = 0.4;
rail_inset = 6;
rail_h = 3;

// =========================
// Sliding lid parameters
// Lid slides in from the BACK.
// The back wall is intentionally lower to create the insertion slot.
// =========================
channel_gap = lid_t + lid_clearance;
back_wall_h = outer_h - channel_gap;
lid_front_gap = 0.4;
lid_w = outer_w - 2 * wall - 2 * rail_inset - 2 * lid_clearance;
lid_d = outer_d - wall - lid_front_gap;

// =========================
// Left side fan parameters
// Change fan_mount_spacing to match your actual fan.
// Example values:
//   ~82.5 for many 92 mm fans
//   ~105 for many 120 mm fans
// =========================
fan_center_y = outer_d / 2;
fan_center_z = outer_h / 2;
fan_opening_d = 96;
fan_mount_spacing = 105;
fan_mount_hole_d = 4.5;

// =========================
// Right side vent field
// =========================
right_vent_area_y = 95;
right_vent_area_z = 85;
right_vent_hole_d = 5;
right_vent_pitch = 12;

// =========================
// Front screen opening
// These are intentionally adjustable because 3.5" Pi displays vary a lot.
// =========================
screen_window_w = 86;
screen_window_h = 56;
screen_center_x = outer_w / 2;
screen_center_z = outer_h / 2;

add_screen_mount_holes = false;
screen_mount_spacing_x = 96;
screen_mount_spacing_z = 66;
screen_mount_hole_d = 3.2;

// =========================
// Back cable slots (generic pass-throughs)
// Tune these to your cable head sizes.
// =========================
hdmi_slot_w = 18;
hdmi_slot_h = 12;
hdmi_slot_center_x = outer_w / 2 - 18;
hdmi_slot_bottom_z = 20;

power_slot_w = 14;
power_slot_h = 12;
power_slot_center_x = outer_w / 2 + 18;
power_slot_bottom_z = 20;

// =========================
// Optional Raspberry Pi standoffs
// Uses the common 4-hole Raspberry Pi mounting pattern as a starting point.
// Verify against your exact board/screen stack before printing.
// =========================
add_pi_standoffs = true;
pi_hole_spacing_x = 58;
pi_hole_spacing_y = 49;
pi_mount_back_offset = 15;   // rear hole row offset from inside back wall
pi_standoff_h = 6;
pi_standoff_od = 7;
pi_standoff_hole_d = 2.9;

// Layout / preview
show_assembled = false;
explode = 15;

module standoff(h = pi_standoff_h, od = pi_standoff_od, id = pi_standoff_hole_d) {
    difference() {
        cylinder(h = h, d = od);
        translate([0, 0, -0.1]) cylinder(h = h + 0.2, d = id);
    }
}

module rounded_slot_x(width, height, depth, radius = 2) {
    rotate([0, 90, 0])
    linear_extrude(height = depth)
    hull() {
        translate([-(width / 2 - radius), -(height / 2 - radius)]) circle(r = radius);
        translate([ (width / 2 - radius), -(height / 2 - radius)]) circle(r = radius);
        translate([-(width / 2 - radius),  (height / 2 - radius)]) circle(r = radius);
        translate([ (width / 2 - radius),  (height / 2 - radius)]) circle(r = radius);
    }
}

module shell_panels() {
    // Bottom
    cube([outer_w, outer_d, bottom_t]);

    // Left wall
    cube([wall, outer_d, outer_h]);

    // Right wall
    translate([outer_w - wall, 0, 0]) cube([wall, outer_d, outer_h]);

    // Front wall
    cube([outer_w, wall, outer_h]);

    // Back wall (lowered to allow lid insertion)
    translate([0, outer_d - wall, 0]) cube([outer_w, wall, back_wall_h]);
}

module lid_rails() {
    rail_z = outer_h - channel_gap - rail_h;
    rail_len = outer_d - wall;  // runs to the back slot

    // Left internal rail
    translate([wall, 0, rail_z])
        cube([rail_inset, rail_len, rail_h]);

    // Right internal rail
    translate([outer_w - wall - rail_inset, 0, rail_z])
        cube([rail_inset, rail_len, rail_h]);
}

module pi_standoffs() {
    x1 = outer_w / 2 - pi_hole_spacing_x / 2;
    x2 = outer_w / 2 + pi_hole_spacing_x / 2;
    y2 = outer_d - wall - pi_mount_back_offset;
    y1 = y2 - pi_hole_spacing_y;

    for (x = [x1, x2])
        for (y = [y1, y2])
            translate([x, y, bottom_t]) standoff();
}

module left_fan_cutouts() {
    // Main airflow opening
    translate([-1, fan_center_y, fan_center_z])
        rounded_slot_x(fan_opening_d, fan_opening_d, wall + 2, radius = 8);

    // Mount holes
    for (yy = [-fan_mount_spacing / 2, fan_mount_spacing / 2])
        for (zz = [-fan_mount_spacing / 2, fan_mount_spacing / 2])
            translate([-1, fan_center_y + yy, fan_center_z + zz])
                rotate([0, 90, 0]) cylinder(h = wall + 2, d = fan_mount_hole_d);
}

module right_side_vents() {
    for (yy = [-(right_vent_area_y / 2) + right_vent_pitch / 2 : right_vent_pitch : (right_vent_area_y / 2) - right_vent_pitch / 2])
        for (zz = [-(right_vent_area_z / 2) + right_vent_pitch / 2 : right_vent_pitch : (right_vent_area_z / 2) - right_vent_pitch / 2])
            translate([outer_w - wall - 1, outer_d / 2 + yy, outer_h / 2 + zz])
                rotate([0, 90, 0]) cylinder(h = wall + 2, d = right_vent_hole_d);
}

module front_screen_cutout() {
    translate([
        screen_center_x - screen_window_w / 2,
        -1,
        screen_center_z - screen_window_h / 2
    ]) cube([screen_window_w, wall + 2, screen_window_h]);

    if (add_screen_mount_holes)
        for (xx = [-screen_mount_spacing_x / 2, screen_mount_spacing_x / 2])
            for (zz = [-screen_mount_spacing_z / 2, screen_mount_spacing_z / 2])
                translate([screen_center_x + xx, wall / 2, screen_center_z + zz])
                    rotate([90, 0, 0]) cylinder(h = wall + 2, d = screen_mount_hole_d);
}

module back_cable_cutouts() {
    translate([hdmi_slot_center_x, outer_d - 1, hdmi_slot_bottom_z + hdmi_slot_h / 2])
        rounded_slot_x(hdmi_slot_w, hdmi_slot_h, wall + 2, radius = 2);

    translate([power_slot_center_x, outer_d - 1, power_slot_bottom_z + power_slot_h / 2])
        rounded_slot_x(power_slot_w, power_slot_h, wall + 2, radius = 2);
}

module case_body() {
    difference() {
        union() {
            shell_panels();
            lid_rails();
            if (add_pi_standoffs) pi_standoffs();
        }
        left_fan_cutouts();
        right_side_vents();
        front_screen_cutout();
        back_cable_cutouts();
    }
}

module top_lid() {
    difference() {
        cube([lid_w, lid_d, lid_t]);

        // Finger notch at rear edge
        translate([lid_w / 2, lid_d + 0.01, lid_t / 2])
            rotate([90, 0, 0]) cylinder(h = 3, d = 22);
    }
}

module assembled() {
    case_body();

    // Lid sits in the side channels just below the top edge.
    translate([
        wall + rail_inset + lid_clearance,
        lid_front_gap,
        outer_h - lid_t
    ]) top_lid();
}

module print_layout() {
    case_body();
    translate([outer_w + 25, 0, 0]) top_lid();
}

if (show_assembled)
    assembled();
else
    print_layout();
