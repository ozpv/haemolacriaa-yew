clean:
	rm -rf frontend/tailwindcss-linux-x64 frontend/Cargo.lock \
    frontend/dist/ frontend/target/ frontend/static/tailwind.css \
    backend/Cargo.lock backend/target/ target/ Cargo.lock

link:
	mkdir -p backend/dist
	ln -s frontend/dist backend/dist
