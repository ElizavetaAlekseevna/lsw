local socket = require("socket")
Camera = require 'Camera'

function love.load()
    love.window.setMode(300, 300)
    udp = socket.udp()
    udp:setpeername("127.0.0.1", 34254)
    udp:settimeout(0)
    t = 0
    id = 0
    updaterate = 0.01
    players = {}
    camera = Camera()
    camera:setFollowStyle('TOPDOWN_TIGHT')
    udp:send("connect")
    repeat
	datas, msg = udp:receive()
	print(datas)
        if datas then
	    id = datas:match("[%s](.*)")
	    players[id] = "0:0"
	    print(id)
	end
    until datas
end

function love.update(dt)
    t = t + dt

    camera:update(dt)
    if players then
	x, y = players[id]:match("(.*):(.*)")
        camera:follow(x, y)
    end
    
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
		_players = {}
		for _id, xy in args:gmatch('["](%d)[, ]+((%-?[%d.e]*):(%-?[%d.e]*))["]') do
		    _players[_id] = xy
		end
		players = _players
		--print(args)
	    end
	end
    until not data
end

function love.draw()
    camera:attach()
    for k, v in pairs(players) do
	x, y = v:match("(.*):(.*)")
        love.graphics.print(k, x, y)
    end
    camera:detach()
    camera:draw()
end

function love.quit()
    udp:send("disconnect")
end
