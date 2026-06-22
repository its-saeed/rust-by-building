// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="intro.html">Introduction</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="projects.html">Projects</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 1 — Caesar Cipher</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/overview.html"><strong aria-hidden="true">1.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/01-print-variables.html"><strong aria-hidden="true">2.</strong> Lesson 1 — Print &amp; Variables</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/02-chars-arithmetic.html"><strong aria-hidden="true">3.</strong> Lesson 2 — Characters &amp; Arithmetic</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/03-functions.html"><strong aria-hidden="true">4.</strong> Lesson 3 — Functions</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/04-iteration-strings.html"><strong aria-hidden="true">5.</strong> Lesson 4 — Iteration &amp; Strings</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/05-control-flow.html"><strong aria-hidden="true">6.</strong> Lesson 5 — Control Flow &amp; Edge Cases</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="caesar/06-final.html"><strong aria-hidden="true">7.</strong> Final Exercise</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 2 — Contact Book</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/overview.html"><strong aria-hidden="true">8.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/01-structs.html"><strong aria-hidden="true">9.</strong> Lesson 1 — Structs</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/02-vec-collections.html"><strong aria-hidden="true">10.</strong> Lesson 2 — Vec &amp; Collections</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/03-methods-impl.html"><strong aria-hidden="true">11.</strong> Lesson 3 — Methods &amp; impl</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/04-option-search.html"><strong aria-hidden="true">12.</strong> Lesson 4 — Option &amp; Search</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/05-user-input.html"><strong aria-hidden="true">13.</strong> Lesson 5 — User Input</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contact-book/06-final.html"><strong aria-hidden="true">14.</strong> Final Exercise</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 3 — Cipher Cracker</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/overview.html"><strong aria-hidden="true">15.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/01-hashmap.html"><strong aria-hidden="true">16.</strong> Lesson 1 — HashMap</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/02-traits.html"><strong aria-hidden="true">17.</strong> Lesson 2 — Traits</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/03-iterators.html"><strong aria-hidden="true">18.</strong> Lesson 3 — Iterators</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/04-enums.html"><strong aria-hidden="true">19.</strong> Lesson 4 — Enums</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/05-frequency-analysis.html"><strong aria-hidden="true">20.</strong> Lesson 5 — Frequency Analysis</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/06-user-input.html"><strong aria-hidden="true">21.</strong> Lesson 6 — User Input &amp; Command Loop</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="cipher-cracker/07-final.html"><strong aria-hidden="true">22.</strong> Final Exercise</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 4 — A Ball Moves</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/overview.html"><strong aria-hidden="true">23.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/physics-primer.html"><strong aria-hidden="true">24.</strong> Physics Primer</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/01-hello-macroquad.html"><strong aria-hidden="true">25.</strong> Lesson 1 — Hello macroquad</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/02-vec2.html"><strong aria-hidden="true">26.</strong> Lesson 2 — Vec2</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/03-operator-overloading.html"><strong aria-hidden="true">27.</strong> Lesson 3 — Operator Overloading</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/04-body.html"><strong aria-hidden="true">28.</strong> Lesson 4 — The Body</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/05-integration.html"><strong aria-hidden="true">29.</strong> Lesson 5 — Integration</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ball-moves/06-wall-bouncing.html"><strong aria-hidden="true">30.</strong> Lesson 6 — Wall Bouncing</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 5 — Many Bodies</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/overview.html"><strong aria-hidden="true">31.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/ownership.html"><strong aria-hidden="true">32.</strong> Ownership in Plain English</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/01-the-world.html"><strong aria-hidden="true">33.</strong> Lesson 1 — The World</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/iterators.html"><strong aria-hidden="true">34.</strong> Iterators</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/02-step.html"><strong aria-hidden="true">35.</strong> Lesson 2 — World::step()</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/03-boundaries.html"><strong aria-hidden="true">36.</strong> Lesson 3 — Boundaries</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/04-gravity.html"><strong aria-hidden="true">37.</strong> Lesson 4 — Gravity</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/05-spawning.html"><strong aria-hidden="true">38.</strong> Lesson 5 — Spawning</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="many-bodies/06-polish.html"><strong aria-hidden="true">39.</strong> Lesson 6 — Polish</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 6 — Balls Collide</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/overview.html"><strong aria-hidden="true">40.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/01-overlap.html"><strong aria-hidden="true">41.</strong> Lesson 1 — Overlap Detection</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/02-collision-type.html"><strong aria-hidden="true">42.</strong> Lesson 2 — The Collision Type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/03-separation.html"><strong aria-hidden="true">43.</strong> Lesson 3 — Separating Bodies</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/collision-physics.html"><strong aria-hidden="true">44.</strong> Collision Physics</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/04-velocity-response.html"><strong aria-hidden="true">45.</strong> Lesson 4 — Velocity Response</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/05-mass.html"><strong aria-hidden="true">46.</strong> Lesson 5 — Mass</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="collisions/06-wiring.html"><strong aria-hidden="true">47.</strong> Lesson 6 — Wiring It Up</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 7 — Pong</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/overview.html"><strong aria-hidden="true">48.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/01-static-scene.html"><strong aria-hidden="true">49.</strong> Lesson 1 — Static Scene</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/02-paddle-movement.html"><strong aria-hidden="true">50.</strong> Lesson 2 — Paddle Movement</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/03-ball.html"><strong aria-hidden="true">51.</strong> Lesson 3 — The Ball</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/04-collision.html"><strong aria-hidden="true">52.</strong> Lesson 4 — Collision Detection</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/05-scoring.html"><strong aria-hidden="true">53.</strong> Lesson 5 — Scoring</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/06-lifetimes.html"><strong aria-hidden="true">54.</strong> Lifetimes</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/07-sprites.html"><strong aria-hidden="true">55.</strong> Lesson 7 — Sprites</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/08-trail.html"><strong aria-hidden="true">56.</strong> Lesson 8 — Ball Trail</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="pong/09-sound.html"><strong aria-hidden="true">57.</strong> Lesson 9 — Sound and Polish</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 8 — Peggle Nights</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/overview.html"><strong aria-hidden="true">58.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/modules.html"><strong aria-hidden="true">59.</strong> Modules</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/rapier-primer.html"><strong aria-hidden="true">60.</strong> Rapier Primer</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/01-static-scene.html"><strong aria-hidden="true">61.</strong> Lesson 1 — Static Scene</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/02-rapier-setup.html"><strong aria-hidden="true">62.</strong> Lesson 2 — Rapier Setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/03-aiming.html"><strong aria-hidden="true">63.</strong> Lesson 3 — Aiming</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/04-trajectory.html"><strong aria-hidden="true">64.</strong> Lesson 4 — Trajectory Preview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/05-ball.html"><strong aria-hidden="true">65.</strong> Lesson 5 — The Ball</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/06-collisions.html"><strong aria-hidden="true">66.</strong> Lesson 6 — Collision Events</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/07-bucket.html"><strong aria-hidden="true">67.</strong> Lesson 7 — The Bucket</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/08-score-states.html"><strong aria-hidden="true">68.</strong> Lesson 8 — Score &amp; States</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="peggle/09-polish.html"><strong aria-hidden="true">69.</strong> Lesson 9 — Polish</a></span></li><li class="chapter-item expanded "><li class="part-title">How Networks Work</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/01-what-is-a-network.html"><strong aria-hidden="true">70.</strong> Chapter 1 — What is a Network?</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/02-layers.html"><strong aria-hidden="true">71.</strong> Chapter 2 — The Layered Model</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/03-addresses.html"><strong aria-hidden="true">72.</strong> Chapter 3 — Addresses and Names</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/04-tcp.html"><strong aria-hidden="true">73.</strong> Chapter 4 — TCP — Reliable Streams</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/05-udp.html"><strong aria-hidden="true">74.</strong> Chapter 5 — UDP — Fast Datagrams</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/06-http.html"><strong aria-hidden="true">75.</strong> Chapter 6 — HTTP and the Web</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/07-sockets.html"><strong aria-hidden="true">76.</strong> Chapter 7 — Sockets</a></span></li><li class="chapter-item expanded "><li class="part-title">Network Programming in Rust</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/08-tcp-server.html"><strong aria-hidden="true">77.</strong> Lesson 1 — TCP Echo Server</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/09-reqwest.html"><strong aria-hidden="true">78.</strong> Lesson 2 — HTTP Client with reqwest</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/10-tcp-client.html"><strong aria-hidden="true">79.</strong> Lesson 3 — TCP Client</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/11-udp.html"><strong aria-hidden="true">80.</strong> Lesson 4 — UDP</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="networking/12-raw-http.html"><strong aria-hidden="true">81.</strong> Mini Project — HTTP over Raw TCP</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 9 — Tele-Sketch</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/overview.html"><strong aria-hidden="true">82.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/01-protocol.html"><strong aria-hidden="true">83.</strong> Lesson 1 — Protocol &amp; Project Setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/02-server.html"><strong aria-hidden="true">84.</strong> Lesson 2 — The Server</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/03-canvas.html"><strong aria-hidden="true">85.</strong> Lesson 3 — Basic Canvas</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/04-going-live.html"><strong aria-hidden="true">86.</strong> Lesson 4 — Going Live</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/05-palette.html"><strong aria-hidden="true">87.</strong> Lesson 5 — Colour Palette</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tele-sketch/06-brush.html"><strong aria-hidden="true">88.</strong> Lesson 6 — Brush Size &amp; Polish</a></span></li><li class="chapter-item expanded "><li class="part-title">How Concurrency Works</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/01-cpu-memory.html"><strong aria-hidden="true">89.</strong> Chapter 1 — The CPU and Memory</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/02-operating-system.html"><strong aria-hidden="true">90.</strong> Chapter 2 — What the Operating System Does</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/03-process-memory.html"><strong aria-hidden="true">91.</strong> Chapter 3 — How a Process Lives in Memory</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/04-scheduling.html"><strong aria-hidden="true">92.</strong> Chapter 4 — Scheduling: The Illusion of Simultaneity</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/05-threads.html"><strong aria-hidden="true">93.</strong> Chapter 5 — Threads: Multiple Stacks, One Heap</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/06-why-threads.html"><strong aria-hidden="true">94.</strong> Chapter 6 — Why We Need Threads</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/07-races-deadlocks.html"><strong aria-hidden="true">95.</strong> Chapter 7 — The Problems: Races and Deadlocks</a></span></li><li class="chapter-item expanded "><li class="part-title">Threading in Rust</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/closures.html"><strong aria-hidden="true">96.</strong> Closures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/08-spawn-join.html"><strong aria-hidden="true">97.</strong> Lesson 1 — Threads and Spawn</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/09-channels.html"><strong aria-hidden="true">98.</strong> Lesson 2 — Message Passing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/10-parallel-weather.html"><strong aria-hidden="true">99.</strong> Mini Project 1 — Parallel Weather</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="threading/11-file-search.html"><strong aria-hidden="true">100.</strong> Mini Project 2 — Parallel File Search</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 10 — Mandelbrot</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="mandelbrot/overview.html"><strong aria-hidden="true">101.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="mandelbrot/complex-numbers.html"><strong aria-hidden="true">102.</strong> Complex Numbers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="mandelbrot/01-serial.html"><strong aria-hidden="true">103.</strong> Lesson 1 — Serial Renderer</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="mandelbrot/02-parallel.html"><strong aria-hidden="true">104.</strong> Lesson 2 — Parallel Renderer</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="mandelbrot/03-zoom.html"><strong aria-hidden="true">105.</strong> Lesson 3 — Zoom</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 11 — Chat Server</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="chat-server/overview.html"><strong aria-hidden="true">106.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="chat-server/01-accept.html"><strong aria-hidden="true">107.</strong> Lesson 1 — Accept and Echo</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="chat-server/02-broadcast.html"><strong aria-hidden="true">108.</strong> Lesson 2 — Broadcasting with Channels</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="chat-server/03-client.html"><strong aria-hidden="true">109.</strong> Lesson 3 — The Client</a></span></li><li class="chapter-item expanded "><li class="part-title">How Async Works</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/01-cost-of-threads.html"><strong aria-hidden="true">110.</strong> Chapter 1 — The Cost of Threads at Scale</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/02-nonblocking-io.html"><strong aria-hidden="true">111.</strong> Chapter 2 — Non-blocking I/O</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/03-event-loop.html"><strong aria-hidden="true">112.</strong> Chapter 3 — The Event Loop</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/04-futures.html"><strong aria-hidden="true">113.</strong> Chapter 4 — Futures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/05-async-await.html"><strong aria-hidden="true">114.</strong> Chapter 5 — async/await</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/06-runtime.html"><strong aria-hidden="true">115.</strong> Chapter 6 — The Runtime</a></span></li><li class="chapter-item expanded "><li class="part-title">Async in Rust</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/07-first-async.html"><strong aria-hidden="true">116.</strong> Lesson 1 — First Async Program</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/08-tasks.html"><strong aria-hidden="true">117.</strong> Lesson 2 — Tasks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/09-async-io.html"><strong aria-hidden="true">118.</strong> Lesson 3 — Async I/O</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/10-async-channels.html"><strong aria-hidden="true">119.</strong> Lesson 4 — Async Channels</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/11-join-select.html"><strong aria-hidden="true">120.</strong> Lesson 5 — join! and select!</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/12-async-weather.html"><strong aria-hidden="true">121.</strong> Mini Project 1 — Async Weather</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async/13-async-file-search.html"><strong aria-hidden="true">122.</strong> Mini Project 2 — Async File Search</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 14 — Async Chat Server</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async-chat/overview.html"><strong aria-hidden="true">123.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async-chat/01-accept.html"><strong aria-hidden="true">124.</strong> Lesson 1 — Accept and Spawn</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="async-chat/02-broadcast.html"><strong aria-hidden="true">125.</strong> Lesson 2 — Broadcasting</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 15 — Download Manager</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="download-manager/overview.html"><strong aria-hidden="true">126.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="download-manager/01-single-file.html"><strong aria-hidden="true">127.</strong> Lesson 1 — Single File</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="download-manager/02-concurrent.html"><strong aria-hidden="true">128.</strong> Lesson 2 — Concurrent Downloads</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="download-manager/03-racing-mirrors.html"><strong aria-hidden="true">129.</strong> Lesson 3 — Racing Mirrors</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="download-manager/04-progress.html"><strong aria-hidden="true">130.</strong> Lesson 4 — Progress Bars</a></span></li><li class="chapter-item expanded "><li class="part-title">Project 12 — Stable World</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="stable-world/overview.html"><strong aria-hidden="true">131.</strong> Overview</a></span></li><li class="chapter-item expanded "><li class="part-title">Appendix</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="appendix-offline-docs.html"><strong aria-hidden="true">132.</strong> Reading stdlib docs offline</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="appendix-rbb.html"><strong aria-hidden="true">133.</strong> Using rbb effectively</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="appendix-omitted.html"><strong aria-hidden="true">134.</strong> What we left out</a></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            // Check both with and without the '.html' suffix to be robust against pretty URLs
            if (link.href.replace(/\.html$/, '') === current_page.replace(/\.html$/, '')
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

