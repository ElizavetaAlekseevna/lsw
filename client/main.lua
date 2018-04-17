local socket = require("socket")

function love.load()
    love.window.setMode(300, 300)
    udp = socket.udp()
    udp:setpeername("127.0.0.1", 34254)
    udp:settimeout(0)
    t = 0
    id = 0
    updaterate = 0.1
    players = {}
    udp:send("connect")
    repeat
	datas, msg = udp:receive()
	print(datas)
        if datas then
	    id = datas:match("*%d*")
	    print(datas)
	end
    until datas
end

function love.update(dt)
    t = t + dt
    if t > updaterate then
        if love.keyboard.isDown('up') then 	
	    udp:send("update move up")
	end
	if love.keyboard.isDown('down') then 
	    udp:send("update move down")
	end
	if love.keyboard.isDown('left') then
	    udp:send("update move left")
	end
	if love.keyboard.isDown('right') then 
	    udp:send("update move right")
	end
	t = t - updaterate
    end

    repeat
	data, msg = udp:receive()
        if data then
	    cmd, args = data:match("(%S*)[%s]+[[](.*)[]]")
	    if cmd == "update" then
		for id, xy in args:gmatch('["](%d)[, ]+((%-?[%d.e]*):(%-?[%d.e]*))["]') do
		    print(id)
		    print(xy)
		    players[id] = xy
		end
		--print(args)
	    end
	end
    until not data
end

function love.draw()
    for k, v in pairs(players) do
	x, y = v:match("(.*):(.*)")
        love.graphics.print(k, x, y)
    end
end

