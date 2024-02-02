# -*- coding: utf-8 -*-

"""
    FixedResolver - example resolver which responds with fixed response
                    to all requests
"""

from dnslib import RR,A,QTYPE
from dnslib.server import DNSServer,DNSHandler,BaseResolver,DNSLogger

def decode_ip(qn):
    components = qn.split('.')
    print('components:', components)
    if not components: return None
    if components[1:3] != ['k37713','xyz']: return None
    items = components[0].split('-')
    if len(items) != 4: return None
    for i in items:
        if int(i) >= 256: return None
    return '.'.join(items)

class TestResolver:
    def resolve(self,request,handler):
        reply = request.reply()
        qname = str(request.q.qname)
        
        ip = decode_ip(qname)
        print(qname, ip)
        reply.add_answer(*RR.fromZone(f"{qname} 60 A {ip}"))
        return reply

resolver = TestResolver()
server = DNSServer(resolver,port=53,address="0.0.0.0")
server.start()
