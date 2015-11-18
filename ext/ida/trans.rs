import back/zmain.h

void main():
	ifn proc()
		printl 'error'
	
rbool proc():
	s=rfile.read_all_n(rf.get_param.get_right)
	if s.empty
		return false
	tsh sh
	rbuf<tword> v
	ifn yword.analyse(sh,s,v,null)
		return false
	return trans(sh,v)
	
rbool trans(tsh& sh,rbuf<tword>& v):
	rset<tfunc> sfunc
	for i=0;i<v.count;i++
		ifn v[i].val==rsoptr(c_sbk_r)&&v.get(i+1).val==rsoptr(c_bbk_l)
			continue
		bbk_right=ybase.find_symm_bbk(sh,v,i+1)
		if bbk_right>=v.count
			continue
		sbk_left=-1
		for j=i-1;j>=0;j--
			if v[j].val==rsoptr(c_sbk_l)
				sbk_left=j
			if v[j].pos!=v[i].pos
				break
		if sbk_left<0
			continue
		if sbk_left+2==i&&v.get(sbk_left+1).val=='void'
			v[sbk_left+1].val.clear
		if v.get(sbk_left+2).val=='argc'&&v.get(i-1).val=='envp'
			ybase.clear_word_val(v,sbk_left+1,i)
		tfunc item
		ifn v.get(sbk_left-1).is_name
			continue
		item.name_dec=v.get(sbk_left-1).val
		if item.name_dec=='if'||item.name_dec=='while'||item.name_dec=='for'
			continue
		item.vword=v.sub(j+1,bbk_right+1)
		//printl item.name_dec
		sfunc.insert_c(item)
	return build(sh,sfunc)
	
rbool build(tsh& sh,rset<tfunc>& sfunc):
	tfunc item
	item.name_dec='main'
	pmain=sfunc.find(item)
	if pmain==null
		return false
	rbuf<tword>& v=pmain->vword
	for i=0;i<v.count;i++
		if v[i].val=='__main'&&v.get(i+1).val=='('&&v.get(i+2).val==')'&&v.get(i+3).val==';'
			ybase.clear_word_val(v,i,i+4)
	result='#define signed\n'
	result+='#define __cdecl\n'
	result+='\n'
	ifn build_one(sh,sfunc,*pmain,result)
		return false
	return rfile.write_all_n('..\\ext\\ida\\test.rs',result)

rbool build_one(tsh& sh,rset<tfunc>& sfunc,tfunc& tfi,rstr& result):
	if tfi.vword.empty
		return true
	i=tfi.vword.find(rsoptr(c_bbk_l))
	if i>=tfi.vword.count
		return false
	for ;i<tfi.vword.count;i++
		tfunc item
		item.name_dec=tfi.vword[i].val
		ptfi=sfunc.find(item)
		if ptfi==null
			continue
		ifn build_one(sh,sfunc,*ptfi,result)
			return false
	result+=ybase.vword_to_s(tfi.vword)
	result+='\n'
	tfi.vword.clear
	return true
